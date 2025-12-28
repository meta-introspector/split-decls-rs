macro_rules! deps {
    () => {
        PunycodeCaller!();
        InternalCaller!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl PunycodeCaller for InternalCaller { const EXTERNAL_CALLER : bool = false ; }
    };
}

impl_23!();