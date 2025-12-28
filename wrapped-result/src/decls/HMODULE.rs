macro_rules! HMODULE {
    () => {
        pub type HMODULE = * mut core :: ffi :: c_void ;
    };
}

HMODULE!()