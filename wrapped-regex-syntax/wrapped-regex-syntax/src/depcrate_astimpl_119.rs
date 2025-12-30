// Generated macro for impl_119 (impl)
macro_rules! Depcrate_astimpl_119 {
() => {
// Module: crate::ast
// Provides: {"impl_119"}
// Dependencies: {}
impl RepetitionRange { # [doc = " Returns true if and only if this repetition range is valid."] # [doc = ""] # [doc = " The only case where a repetition range is invalid is if it is bounded"] # [doc = " and its start is greater than its end."] pub fn is_valid (& self) -> bool { match * self { RepetitionRange :: Bounded (s , e) if s > e => false , _ => true , } } }
};
}
