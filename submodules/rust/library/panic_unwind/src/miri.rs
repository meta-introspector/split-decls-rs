mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkitem!{type Payload = Box < Box < dyn Any + Send > > ;}
mkitem!{unsafe extern "Rust" { # [doc = " Miri-provided extern function to begin unwinding."] fn miri_start_unwind (payload : * mut u8) -> ! ; }}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    pub (crate) unsafe fn panic (payload : Box < dyn Any + Send >) -> u32 { let payload_box : Payload = Box :: new (payload) ; unsafe { miri_start_unwind (Box :: into_raw (payload_box) as * mut u8) } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) unsafe fn cleanup (payload_box : * mut u8) -> Box < dyn Any + Send > { let payload_box : Payload = unsafe { Box :: from_raw (payload_box as * mut _) } ; * payload_box }
}