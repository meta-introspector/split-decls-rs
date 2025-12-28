macro_rules! deps {
    () => {
        Eager!();
        BufferKind!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl BufferKind for Eager { }
    };
}

impl_16!();