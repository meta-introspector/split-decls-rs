// Generated macro for rustc_link_arg_bins (function)
macro_rules! Depcrate_outputrustc_link_arg_bins {
() => {
// Module: crate::output
// Provides: {"rustc_link_arg_bins"}
// Dependencies: {}
# [doc = " The `rustc-link-arg-bins` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " the binary target."] # [doc = ""] # [doc = " Its usage is highly platform specific. It is useful to set"] # [doc = " a linker script or other linker options."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_link_arg_bins (flag : & str) { if flag . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-arg-bins: invalid flag {flag:?}") ; } emit ("rustc-link-arg-bins" , flag) ; }
};
}
