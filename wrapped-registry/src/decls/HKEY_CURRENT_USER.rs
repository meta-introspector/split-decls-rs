macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! HKEY_CURRENT_USER {
    () => {
        deps!();
        pub const HKEY_CURRENT_USER : HKEY = - 2147483647i32 as _ ;
    };
}

HKEY_CURRENT_USER!();