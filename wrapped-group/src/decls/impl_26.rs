macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < G : Group > Wnaf < () , Vec < G > , Vec < i64 > > { # [doc = " Construct a new wNAF context without allocating."] pub fn new () -> Self { Wnaf { base : vec ! [] , scalar : vec ! [] , window_size : () , } } }
    };
}

impl_26!()