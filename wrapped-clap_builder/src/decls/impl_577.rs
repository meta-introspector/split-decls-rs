macro_rules! deps {
    () => {
        Stream!();
        Colorizer!();
        StyledStr!();
        ColorChoice!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl Colorizer { pub (crate) fn new (stream : Stream , color_when : ColorChoice) -> Self { Colorizer { stream , color_when , content : Default :: default () , } } pub (crate) fn with_content (mut self , content : StyledStr) -> Self { self . content = content ; self } }
    };
}

impl_577!();