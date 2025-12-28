macro_rules! build_string {
    () => {
        pub (crate) fn build_string (f : impl FnOnce (& RustString)) -> Result < String , FromUtf8Error > { String :: from_utf8 (RustString :: build_byte_buffer (f)) }
    };
}

build_string!()