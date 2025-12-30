// Generated macro for println (macro)
macro_rules! Depcrate_macrosprintln {
() => {
// Module: crate::macros
// Provides: {"println"}
// Dependencies: {}
# [doc = " Prints to the standard output, with a newline."] # [doc = ""] # [doc = " Adapted from [`std::println`]."] # [doc = ""] # [doc = " [`std::println`]: https://doc.rust-lang.org/stable/std/macro.println.html"] # [cfg (target_os = "none")] # [macro_export] macro_rules ! println { () => { $ crate :: print ! ("\n") } ; ($ ($ arg : tt) *) => { { $ crate :: console :: _print (:: core :: format_args ! ("{}\n" , format_args ! ($ ($ arg) *))) ; } } ; }
};
}
