macro_rules! deps {
    () => {
        ErrorDetail!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a > ErrorDetail < 'a > { pub fn new (input : & 'a str , message : impl Into < String >) -> Self { let input = & input [.. input . len () . min (1)] ; Self { input , message : message . into () } } }
    };
}

impl_92!();