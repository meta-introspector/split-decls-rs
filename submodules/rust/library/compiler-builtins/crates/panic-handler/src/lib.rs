
macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    # [panic_handler] fn panic (_ : & core :: panic :: PanicInfo < '_ >) -> ! { loop { } }
}