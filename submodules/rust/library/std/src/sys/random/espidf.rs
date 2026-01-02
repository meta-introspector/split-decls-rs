mkuse!{use crate :: ffi :: c_void ;}
mkitem!{unsafe extern "C" { fn esp_fill_random (buf : * mut c_void , len : usize) ; }}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { esp_fill_random (bytes . as_mut_ptr () . cast () , bytes . len ()) } }
}