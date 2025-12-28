macro_rules! PCWSTR {
    () => {
        pub type PCWSTR = * const u16 ;
    };
}

PCWSTR!();