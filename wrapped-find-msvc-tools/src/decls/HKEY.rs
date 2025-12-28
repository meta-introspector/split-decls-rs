macro_rules! HKEY {
    () => {
        pub type HKEY = * mut core :: ffi :: c_void ;
    };
}

HKEY!()