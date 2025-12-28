macro_rules! HANDLE {
    () => {
        pub type HANDLE = * mut core :: ffi :: c_void ;
    };
}

HANDLE!()