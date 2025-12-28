macro_rules! build_byte_buffer {
    () => {
        pub (crate) fn build_byte_buffer (f : impl FnOnce (& RustString)) -> Vec < u8 > { RustString :: build_byte_buffer (f) }
    };
}

build_byte_buffer!()