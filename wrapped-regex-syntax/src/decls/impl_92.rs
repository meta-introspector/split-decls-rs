macro_rules! deps {
    () => {
        Literal!();
        ClassSetItem!();
        Range!();
        Span!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl ClassSetItem { # [doc = " Return the span of this character class set item."] pub fn span (& self) -> & Span { match * self { ClassSetItem :: Empty (ref span) => span , ClassSetItem :: Literal (ref x) => & x . span , ClassSetItem :: Range (ref x) => & x . span , ClassSetItem :: Ascii (ref x) => & x . span , ClassSetItem :: Perl (ref x) => & x . span , ClassSetItem :: Unicode (ref x) => & x . span , ClassSetItem :: Bracketed (ref x) => & x . span , ClassSetItem :: Union (ref x) => & x . span , } } }
    };
}

impl_92!();