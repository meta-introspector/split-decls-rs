// Generated macro for rustc_cdylib_link_arg (function)
macro_rules! Depcrate_outputrustc_cdylib_link_arg {
() => {
// Module: crate::output
// Provides: {"rustc_cdylib_link_arg"}
// Dependencies: {}
# [doc = " The `rustc-cdylib-link-arg` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " a `cdylib` library target."] # [doc = ""] # [doc = " Its usage is highly platform specific. It is useful"] # [doc = " to set the shared library version or the runtime-path."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_cdylib_link_arg (flag : & str) { if flag . contains ('\n') { panic ! ("cannot emit rustc-cdylib-link-arg: invalid flag {flag:?}") ; } emit ("rustc-cdylib-link-arg" , flag) ; }
};
}
