mkitem!{# [link (name = "zircon")] unsafe extern "C" { fn zx_cprng_draw (buffer : * mut u8 , len : usize) ; }}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { zx_cprng_draw (bytes . as_mut_ptr () , bytes . len ()) } }
}