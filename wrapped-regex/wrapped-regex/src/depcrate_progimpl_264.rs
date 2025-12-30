// Generated macro for impl_264 (impl)
macro_rules! Depcrate_progimpl_264 {
() => {
// Module: crate::prog
// Provides: {"impl_264"}
// Dependencies: {}
impl Inst { # [doc = " Returns true if and only if this is a match instruction."] pub fn is_match (& self) -> bool { match * self { Inst :: Match (_) => true , _ => false , } } }
};
}
