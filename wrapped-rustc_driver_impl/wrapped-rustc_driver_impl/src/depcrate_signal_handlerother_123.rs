// Generated macro for other_123 (other)
macro_rules! Depcrate_signal_handlerother_123 {
() => {
// Module: crate::signal_handler
// Provides: {"other_123"}
// Dependencies: {}
# [doc = " We don't really care how many bytes we actually get out. SIGSEGV comes for our head."] # [doc = " Splash stderr with letters of our own blood to warn our friends about the monster."] macro raw_errln ($ tokens : tt) { let _ = :: core :: fmt :: Write :: write_fmt (& mut RawStderr (()) , format_args ! ($ tokens)) ; let _ = :: core :: fmt :: Write :: write_char (& mut RawStderr (()) , '\n') ; }
};
}
