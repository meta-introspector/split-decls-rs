// Generated macro for _panic_print (function)
macro_rules! Depcrate_console_panic_print {
() => {
// Module: crate::console
// Provides: {"_panic_print"}
// Dependencies: {}
# [doc (hidden)] pub fn _panic_print (args : fmt :: Arguments < '_ >) { let mut console = unsafe { CONSOLE . make_guard_unchecked () } ; console . write_fmt (args) . ok () ; mem :: forget (console) ; }
};
}
