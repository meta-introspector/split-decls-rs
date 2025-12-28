macro_rules! deps {
    () => {
        LenType!();
    };
}

macro_rules! check_capacity_fits {
    () => {
        deps!();
        pub const fn check_capacity_fits < LenT : LenType , const N : usize > () { assert ! (LenT :: MAX_USIZE >= N , "The capacity is larger than `LenT` can hold, increase the size of `LenT` or reduce the capacity") ; }
    };
}

check_capacity_fits!();