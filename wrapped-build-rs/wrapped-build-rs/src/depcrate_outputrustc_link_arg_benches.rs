// Generated macro for rustc_link_arg_benches (function)
macro_rules! Depcrate_outputrustc_link_arg_benches {
() => {
// Module: crate::output
// Provides: {"rustc_link_arg_benches"}
// Dependencies: {}
# [doc = " The `rustc-link-arg-benches` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " a benchmark target."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_link_arg_benches (flag : & str) { if flag . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-arg-benches: invalid flag {flag:?}") ; } emit ("rustc-link-arg-benches" , flag) ; }
};
}
