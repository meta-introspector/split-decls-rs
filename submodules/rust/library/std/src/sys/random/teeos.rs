mkitem!{unsafe extern "C" { fn TEE_GenerateRandom (randomBuffer : * mut core :: ffi :: c_void , randomBufferLen : libc :: size_t) ; }}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { TEE_GenerateRandom (bytes . as_mut_ptr () . cast () , bytes . len ()) } }
}