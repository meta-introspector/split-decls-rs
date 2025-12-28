macro_rules! PCSTR {
    () => {
        pub type PCSTR = * const u8 ;
    };
}

PCSTR!();