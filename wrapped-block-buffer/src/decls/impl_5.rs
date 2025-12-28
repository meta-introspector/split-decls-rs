macro_rules! deps {
    () => {
        Eager!();
        BufferKind!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl BufferKind for Eager { }
    };
}

impl_5!()