mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkitem!{unsafe extern "Rust" { # [rustc_std_internal_symbol] safe fn __rust_abort () -> !; }}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) unsafe fn cleanup (_ptr : * mut u8) -> Box < dyn Any + Send > { __rust_abort () }
}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    pub (crate) unsafe fn panic (_data : Box < dyn Any + Send >) -> u32 { __rust_abort () }
}