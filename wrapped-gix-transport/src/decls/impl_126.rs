macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl crate :: IsSpuriousError for Error { }
    };
}

impl_126!();