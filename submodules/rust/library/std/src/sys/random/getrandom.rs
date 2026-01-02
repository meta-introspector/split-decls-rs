
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (mut bytes : & mut [u8]) { while ! bytes . is_empty () { let r = unsafe { libc :: getrandom (bytes . as_mut_ptr () . cast () , bytes . len () , 0) } ; assert_ne ! (r , - 1 , "failed to generate random data") ; bytes = & mut bytes [r as usize ..] ; } }
}