mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: panic :: PanicPayload ;}
mkitem!{cfg_select ! { all (target_os = "emscripten" , not (emscripten_wasm_eh)) => { # [path = "emcc.rs"] mod imp ; } target_os = "hermit" => { # [path = "hermit.rs"] mod imp ; } target_os = "l4re" => { # [path = "dummy.rs"] mod imp ; } any (all (target_family = "windows" , target_env = "gnu") , target_os = "psp" , target_os = "xous" , target_os = "solid_asp3" , all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "nuttx"))) , all (target_vendor = "fortanix" , target_env = "sgx") , target_family = "wasm" ,) => { # [path = "gcc.rs"] mod imp ; } miri => { # [path = "miri.rs"] mod imp ; } all (target_env = "msvc" , not (target_arch = "arm")) => { # [path = "seh.rs"] mod imp ; } _ => { # [path = "dummy.rs"] mod imp ; } }}
mkitem!{unsafe extern "C" { # [doc = " Handler in std called when a panic object is dropped outside of"] # [doc = " `catch_unwind`."] # [rustc_std_internal_symbol] fn __rust_drop_panic () -> ! ; # [doc = " Handler in std called when a foreign exception is caught."] # [rustc_std_internal_symbol] fn __rust_foreign_exception () -> ! ; }}

macro_rules! __rust_panic_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_panic_cleanup in module {}", module_path!());
    };
}

mkfn!{
    __rust_panic_cleanup_introspect!();
    # [rustc_std_internal_symbol] # [allow (improper_ctypes_definitions)] pub unsafe extern "C" fn __rust_panic_cleanup (payload : * mut u8) -> * mut (dyn Any + Send + 'static) { unsafe { Box :: into_raw (imp :: cleanup (payload)) } }
}

macro_rules! __rust_start_panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_start_panic in module {}", module_path!());
    };
}

mkfn!{
    __rust_start_panic_introspect!();
    # [rustc_std_internal_symbol] pub unsafe fn __rust_start_panic (payload : & mut dyn PanicPayload) -> u32 { unsafe { let payload = Box :: from_raw (payload . take_box ()) ; imp :: panic (payload) } }
}