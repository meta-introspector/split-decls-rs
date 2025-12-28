macro_rules! deps {
    () => {
        Response!();
        BatchResponse!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < Vec < Response > > for BatchResponse { fn from (responses : Vec < Response >) -> Self { Self :: Batch (responses) } }
    };
}

impl_114!()