macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! HKEY_CURRENT_CONFIG {
    () => {
        deps!();
        pub const HKEY_CURRENT_CONFIG : HKEY = - 2147483643i32 as _ ;
    };
}

HKEY_CURRENT_CONFIG!()