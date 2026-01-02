mkuse!{# [cfg (not (any (target_os = "haiku" , target_os = "illumos" , target_os = "solaris" , target_os = "vita" ,)))] use libc :: arc4random_buf ;}
mkitem!{# [cfg (any (target_os = "haiku" , target_os = "illumos" , target_os = "solaris" , target_os = "vita" ,))] # [cfg_attr (target_os = "haiku" , link (name = "bsd"))] unsafe extern "C" { fn arc4random_buf (buf : * mut core :: ffi :: c_void , nbytes : libc :: size_t) ; }}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { arc4random_buf (bytes . as_mut_ptr () . cast () , bytes . len ()) } }
}