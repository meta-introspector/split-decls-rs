macro_rules! deps {
    () => {
        OLECHAR!();
    };
}

macro_rules! LPCOLESTR {
    () => {
        deps!();
        pub type LPCOLESTR = * const OLECHAR ;
    };
}

LPCOLESTR!();