macro_rules! deps {
    () => {
        FpResult!();
        Status!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T > FpResult < T > { pub fn new (val : T , status : Status) -> Self { Self { val , status } } # [doc = " Return `val` with `Status::OK`."] pub fn ok (val : T) -> Self { Self { val , status : Status :: OK , } } }
    };
}

impl_37!();