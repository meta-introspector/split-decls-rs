
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { wasi :: random_get (bytes . as_mut_ptr () , bytes . len ()) . expect ("failed to generate random data") } }
}