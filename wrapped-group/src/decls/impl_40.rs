macro_rules! deps {
    () => {
        WnafBase!();
        Group!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < G : Group , const WINDOW_SIZE : usize > WnafBase < G , WINDOW_SIZE > { # [doc = " Computes a window table for the given base with the specified `WINDOW_SIZE`."] pub fn new (base : G) -> Self { let mut table = vec ! [] ; wnaf_table (& mut table , base , WINDOW_SIZE) ; WnafBase { table } } }
    };
}

impl_40!();