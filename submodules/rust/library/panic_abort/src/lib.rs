mkmod!{android, { 
                getname!(android);
                getsrc!(android);
                getpath!(android);
                get_deps!(android);
                get_crates!(android);
                mkinclude!(android);
                 
            }}
mkmod!{zkvm, { 
                getname!(zkvm);
                getsrc!(zkvm);
                getpath!(zkvm);
                get_deps!(zkvm);
                get_crates!(zkvm);
                mkinclude!(zkvm);
                 
            }}
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: panic :: PanicPayload ;}

macro_rules! __rust_panic_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_panic_cleanup in module {}", module_path!());
    };
}

mkfn!{
    __rust_panic_cleanup_introspect!();
    # [rustc_std_internal_symbol] # [allow (improper_ctypes_definitions)] pub unsafe extern "C" fn __rust_panic_cleanup (_ : * mut u8) -> * mut (dyn Any + Send + 'static) { unreachable ! () }
}

macro_rules! __rust_start_panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_start_panic in module {}", module_path!());
    };
}

mkfn!{
    __rust_start_panic_introspect!();
    # [rustc_std_internal_symbol] pub unsafe fn __rust_start_panic (_payload : & mut dyn PanicPayload) -> u32 { # [cfg (target_os = "android")] unsafe { android :: android_set_abort_message (_payload) ; } # [cfg (target_os = "zkvm")] unsafe { zkvm :: zkvm_set_abort_message (_payload) ; } unsafe extern "Rust" { # [rustc_std_internal_symbol] safe fn __rust_abort () -> !; } __rust_abort () }
}
mkmod!{personalities, { 
                getname!(personalities);
                getsrc!(personalities);
                getpath!(personalities);
                get_deps!(personalities);
                get_crates!(personalities);
                mkinclude!(personalities);
                mkitem!{# [rustc_std_internal_symbol] # [allow (non_upper_case_globals)] # [cfg (target_os = "emscripten")] static rust_eh_catch_typeinfo : [usize ; 2] = [0 ; 2] ;} 
            }}