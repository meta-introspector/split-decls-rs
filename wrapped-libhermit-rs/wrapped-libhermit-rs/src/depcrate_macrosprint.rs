// Generated macro for print (macro)
macro_rules! Depcrate_macrosprint {
() => {
// Module: crate::macros
// Provides: {"print"}
// Dependencies: {}
# [doc = " Prints to the standard output."] # [doc = ""] # [doc = " Adapted from [`std::print`]."] # [doc = ""] # [doc = " [`std::print`]: https://doc.rust-lang.org/stable/std/macro.print.html"] # [cfg (target_os = "none")] # [macro_export] macro_rules ! print { ($ ($ arg : tt) *) => { { $ crate :: console :: _print (:: core :: format_args ! ($ ($ arg) *)) ; } } ; }
};
}
