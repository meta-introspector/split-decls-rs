mkuse!{use crate :: mem :: transmute ;}

macro_rules! register_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function register in module {}", module_path!());
    };
}

mkfn!{
    register_introspect!();
    pub unsafe fn register (t : * mut u8 , dtor : unsafe extern "C" fn (* mut u8)) { # [doc = " This is necessary because the __cxa_thread_atexit_impl implementation"] # [doc = " std links to by default may be a C or C++ implementation that was not"] # [doc = " compiled using the Clang integer normalization option."] # [cfg (sanitizer_cfi_normalize_integers)] use core :: ffi :: c_int ; # [cfg (not (sanitizer_cfi_normalize_integers))] # [cfi_encoding = "i"] # [repr (transparent)] # [allow (non_camel_case_types)] pub struct c_int (# [allow (dead_code)] pub core :: ffi :: c_int) ; unsafe extern "C" { # [linkage = "extern_weak"] static __dso_handle : * mut u8 ; # [linkage = "extern_weak"] static __cxa_thread_atexit_impl : Option < extern "C" fn (unsafe extern "C" fn (* mut libc :: c_void) , * mut libc :: c_void , * mut libc :: c_void ,) -> c_int , > ; } if let Some (f) = unsafe { __cxa_thread_atexit_impl } { unsafe { f (transmute :: < unsafe extern "C" fn (* mut u8) , unsafe extern "C" fn (* mut libc :: c_void) > (dtor ,) , t . cast () , (& raw const __dso_handle) as * mut _ ,) ; } } else { unsafe { super :: list :: register (t , dtor) ; } } }
}