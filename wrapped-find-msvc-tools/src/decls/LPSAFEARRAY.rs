macro_rules! deps {
    () => {
        SAFEARRAY!();
    };
}

macro_rules! LPSAFEARRAY {
    () => {
        deps!();
        pub type LPSAFEARRAY = * mut SAFEARRAY ;
    };
}

LPSAFEARRAY!()