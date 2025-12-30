// Generated macro for target (function)
macro_rules! Depcrate_inputtarget {
() => {
// Module: crate::input
// Provides: {"target"}
// Dependencies: {}
# [doc = " The [target triple] that is being compiled for. Native code should be compiled"] # [doc = "  for this triple."] # [doc = ""] # [doc = " [target triple]: https://doc.rust-lang.org/stable/cargo/appendix/glossary.html#target"] # [track_caller] pub fn target () -> String { to_string (var_or_panic ("TARGET")) }
};
}
