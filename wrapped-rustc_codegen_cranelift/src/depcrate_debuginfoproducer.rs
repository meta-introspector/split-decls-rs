// Generated macro for producer (function)
macro_rules! Depcrate_debuginfoproducer {
() => {
// Module: crate::debuginfo
// Provides: {"producer"}
// Dependencies: {}
pub (crate) fn producer (sess : & Session) -> String { format ! ("rustc version {} with cranelift {}" , sess . cfg_version , cranelift_codegen :: VERSION) }
};
}
