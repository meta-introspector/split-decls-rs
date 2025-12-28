macro_rules! deps {
    () => {
        ExternalCaller!();
        PunycodeCaller!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl PunycodeCaller for ExternalCaller { const EXTERNAL_CALLER : bool = true ; }
    };
}

impl_25!()