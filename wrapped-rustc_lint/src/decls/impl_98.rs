macro_rules! deps {
    () => {
        InitError!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl InitError { fn spanned (self , span : Span) -> InitError { Self { span : Some (span) , .. self } } fn nested (self , nested : impl Into < Option < InitError > >) -> InitError { assert ! (self . nested . is_none ()) ; Self { nested : nested . into () . map (Box :: new) , .. self } } }
    };
}

impl_98!();