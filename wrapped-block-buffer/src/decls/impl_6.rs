macro_rules! deps {
    () => {
        BufferKind!();
        Lazy!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl BufferKind for Lazy { }
    };
}

impl_6!()