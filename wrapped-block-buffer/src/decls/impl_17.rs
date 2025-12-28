macro_rules! deps {
    () => {
        Lazy!();
        BufferKind!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl BufferKind for Lazy { }
    };
}

impl_17!();