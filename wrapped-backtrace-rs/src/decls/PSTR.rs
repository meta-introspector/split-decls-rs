macro_rules! PSTR {
    () => {
        pub type PSTR = * mut u8 ;
    };
}

PSTR!()