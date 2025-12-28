macro_rules! deps {
    () => {
        StyledChar!();
        ElementStyle!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl StyledChar { pub (crate) const SPACE : Self = StyledChar :: new (' ' , ElementStyle :: NoStyle) ; pub (crate) const fn new (ch : char , style : ElementStyle) -> StyledChar { StyledChar { ch , style } } }
    };
}

impl_87!();