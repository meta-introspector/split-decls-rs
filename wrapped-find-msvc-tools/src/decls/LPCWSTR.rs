macro_rules! deps {
    () => {
        WCHAR!();
    };
}

macro_rules! LPCWSTR {
    () => {
        deps!();
        pub type LPCWSTR = * const WCHAR ;
    };
}

LPCWSTR!();