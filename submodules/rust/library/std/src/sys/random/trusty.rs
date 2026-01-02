mkitem!{unsafe extern "C" { fn trusty_rng_secure_rand (randomBuffer : * mut core :: ffi :: c_void , randomBufferLen : libc :: size_t) ; }}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { trusty_rng_secure_rand (bytes . as_mut_ptr () . cast () , bytes . len ()) } }
}