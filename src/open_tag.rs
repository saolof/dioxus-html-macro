use crate::prelude::*; 



pub struct OpenTag {
    pub open_angle: Token![<],
    pub tagname: Ident,
    pub attributes: Attributes,
    pub fslash: Option<Token![/]>,
    pub close_angle: Token![>],
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let open_angle = input.parse()?;
        let tagname = input.parse()?;
        let attributes = input.parse()?;
        let fslash = input.parse()?;
        let close_angle = input.parse()?;

        Ok(OpenTag {
            open_angle,
            tagname,
            attributes,
            fslash,
            close_angle,
        })
    }
}
