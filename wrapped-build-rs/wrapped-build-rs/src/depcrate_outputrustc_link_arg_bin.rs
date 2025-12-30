// Generated macro for rustc_link_arg_bin (function)
macro_rules! Depcrate_outputrustc_link_arg_bin {
() => {
// Module: crate::output
// Provides: {"rustc_link_arg_bin"}
// Dependencies: {}
# [doc = " The `rustc-link-arg-bin` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " the binary target with name `BIN`. Its usage is highly platform specific."] # [doc = ""] # [doc = " It"] # [doc = " is useful to set a linker script or other linker options."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_link_arg_bin (bin : & str , flag : & str) { if ! is_ident (bin) { panic ! ("cannot emit rustc-link-arg-bin: invalid bin name {bin:?}") ; } if flag . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-arg-bin: invalid flag {flag:?}") ; } emit ("rustc-link-arg-bin" , format_args ! ("{bin}={flag}")) ; }
};
}
