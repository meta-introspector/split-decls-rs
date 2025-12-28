macro_rules! BSTR {
    () => {
        pub type BSTR = * const u16 ;
    };
}

BSTR!();