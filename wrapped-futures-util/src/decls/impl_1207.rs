macro_rules! deps {
    () => {
        WriteHalf!();
        ReadHalf!();
    };
}

macro_rules! impl_1207 {
    () => {
        deps!();
        impl < T > WriteHalf < T > { # [doc = " Checks if this `WriteHalf` and some `ReadHalf` were split from the same stream."] pub fn is_pair_of (& self , other : & ReadHalf < T >) -> bool { self . handle . is_pair_of (& other . handle) } }
    };
}

impl_1207!()