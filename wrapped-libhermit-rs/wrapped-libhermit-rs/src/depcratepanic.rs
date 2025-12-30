// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [cfg (target_os = "none")] # [panic_handler] fn panic (info : & core :: panic :: PanicInfo < '_ >) -> ! { let core_id = crate :: arch :: core_local :: core_id () ; panic_println ! ("[{core_id}][PANIC] {info}\n") ; crate :: scheduler :: shutdown (1) ; }
};
}
