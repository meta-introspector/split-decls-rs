mkuse!{use core :: ffi :: c_void ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sys :: c ;}
mkitem!{unsafe extern "C" { # [link_name = "_tls_used"] static TLS_USED : u8 ; }}

macro_rules! enable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable in module {}", module_path!());
    };
}

mkfn!{
    enable_introspect!();
    pub fn enable () { unsafe { ptr :: from_ref (& TLS_USED) . read_volatile () } ; unsafe { ptr :: from_ref (& CALLBACK) . read_volatile () } ; }
}
mkitem!{# [unsafe (link_section = ".CRT$XLB")] # [cfg_attr (miri , used)] pub static CALLBACK : unsafe extern "system" fn (* mut c_void , u32 , * mut c_void) = tls_callback ;}

macro_rules! tls_callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tls_callback in module {}", module_path!());
    };
}

mkfn!{
    tls_callback_introspect!();
    unsafe extern "system" fn tls_callback (_h : * mut c_void , dw_reason : u32 , _pv : * mut c_void) { if dw_reason == c :: DLL_THREAD_DETACH || dw_reason == c :: DLL_PROCESS_DETACH { unsafe { # [cfg (target_thread_local)] super :: super :: destructors :: run () ; # [cfg (not (target_thread_local))] super :: super :: key :: run_dtors () ; crate :: rt :: thread_cleanup () ; } } }
}