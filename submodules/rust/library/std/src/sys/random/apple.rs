
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { let ret = unsafe { libc :: CCRandomGenerateBytes (bytes . as_mut_ptr () . cast () , bytes . len ()) } ; assert_eq ! (ret , libc :: kCCSuccess , "failed to generate random data") ; }
}