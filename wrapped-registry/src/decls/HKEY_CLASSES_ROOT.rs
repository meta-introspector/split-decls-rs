macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! HKEY_CLASSES_ROOT {
    () => {
        deps!();
        pub const HKEY_CLASSES_ROOT : HKEY = - 2147483648i32 as _ ;
    };
}

HKEY_CLASSES_ROOT!();