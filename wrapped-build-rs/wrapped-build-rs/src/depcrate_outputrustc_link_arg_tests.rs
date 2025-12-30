// Generated macro for rustc_link_arg_tests (function)
macro_rules! Depcrate_outputrustc_link_arg_tests {
() => {
// Module: crate::output
// Provides: {"rustc_link_arg_tests"}
// Dependencies: {}
# [doc = " The `rustc-link-arg-tests` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " a tests target."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_link_arg_tests (flag : & str) { if flag . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-arg-tests: invalid flag {flag:?}") ; } emit ("rustc-link-arg-tests" , flag) ; }
};
}
