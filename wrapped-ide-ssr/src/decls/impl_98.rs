macro_rules! deps {
    () => {
        SsrError!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl SsrError { pub (crate) fn new (message : impl Into < String >) -> SsrError { SsrError (message . into ()) } }
    };
}

impl_98!()