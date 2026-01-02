
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { for chunk in bytes . chunks_mut (256) { let r = unsafe { libc :: getentropy (chunk . as_mut_ptr () . cast () , chunk . len ()) } ; assert_ne ! (r , - 1 , "failed to generate random data") ; } }
}