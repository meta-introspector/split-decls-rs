macro_rules! PWSTR {
    () => {
        pub type PWSTR = * mut u16 ;
    };
}

PWSTR!()