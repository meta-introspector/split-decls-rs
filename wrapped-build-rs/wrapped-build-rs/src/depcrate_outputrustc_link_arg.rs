// Generated macro for rustc_link_arg (function)
macro_rules! Depcrate_outputrustc_link_arg {
() => {
// Module: crate::output
// Provides: {"rustc_link_arg"}
// Dependencies: {}
# [doc = " The `rustc-link-arg` instruction tells Cargo to pass the"] # [doc = " [`-C link-arg=FLAG` option][link-arg] to the compiler, but only when building"] # [doc = " supported targets (benchmarks, binaries, cdylib crates, examples, and tests)."] # [doc = ""] # [doc = " Its usage is highly platform specific. It is useful to set the shared library"] # [doc = " version or linker script."] # [doc = ""] # [doc = " [link-arg]: https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg"] # [track_caller] pub fn rustc_link_arg (flag : & str) { if flag . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-arg: invalid flag {flag:?}") ; } emit ("rustc-link-arg" , flag) ; }
};
}
