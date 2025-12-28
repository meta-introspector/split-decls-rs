macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! HKEY_USERS {
    () => {
        deps!();
        pub const HKEY_USERS : HKEY = - 2147483645i32 as _ ;
    };
}

HKEY_USERS!();