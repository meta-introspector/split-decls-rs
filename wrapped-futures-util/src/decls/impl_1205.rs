macro_rules! deps {
    () => {
        ReadHalf!();
        WriteHalf!();
    };
}

macro_rules! impl_1205 {
    () => {
        deps!();
        impl < T > ReadHalf < T > { # [doc = " Checks if this `ReadHalf` and some `WriteHalf` were split from the same stream."] pub fn is_pair_of (& self , other : & WriteHalf < T >) -> bool { self . handle . is_pair_of (& other . handle) } }
    };
}

impl_1205!();