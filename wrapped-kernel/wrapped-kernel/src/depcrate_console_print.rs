// Generated macro for _print (function)
macro_rules! Depcrate_console_print {
() => {
// Module: crate::console
// Provides: {"_print"}
// Dependencies: {}
# [doc (hidden)] pub fn _print (args : fmt :: Arguments < '_ >) { CONSOLE . lock () . write_fmt (args) . unwrap () ; }
};
}
