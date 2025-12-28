macro_rules! HINSTANCE {
    () => {
        pub type HINSTANCE = * mut core :: ffi :: c_void ;
    };
}

HINSTANCE!();