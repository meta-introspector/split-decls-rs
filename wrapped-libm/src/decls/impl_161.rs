macro_rules! deps {
    () => {
        Status!();
        FpResult!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < T > FpResult < T > { pub fn new (val : T , status : Status) -> Self { Self { val , status } } # [doc = " Return `val` with `Status::OK`."] pub fn ok (val : T) -> Self { Self { val , status : Status :: OK , } } }
    };
}

impl_161!()