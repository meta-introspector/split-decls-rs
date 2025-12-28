macro_rules! deps {
    () => {
        Colorizer!();
        StyledStr!();
        ColorChoice!();
        Stream!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl Colorizer { pub (crate) fn new (stream : Stream , color_when : ColorChoice) -> Self { Colorizer { stream , color_when , content : Default :: default () , } } pub (crate) fn with_content (mut self , content : StyledStr) -> Self { self . content = content ; self } }
    };
}

impl_577!()