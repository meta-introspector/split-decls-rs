macro_rules! deps {
    () => {
        SsrError!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl std :: error :: Error for SsrError { }
    };
}

impl_193!()