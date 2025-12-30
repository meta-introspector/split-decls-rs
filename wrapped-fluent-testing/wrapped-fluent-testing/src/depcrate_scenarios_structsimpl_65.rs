// Generated macro for impl_65 (impl)
macro_rules! Depcrate_scenarios_structsimpl_65 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_65"}
// Dependencies: {}
impl From < (& str , & str) > for Query { fn from (i : (& str , & str)) -> Self { Self { input : i . 0 . into () , output : Some (i . 1 . into ()) , exceptional_context : ExceptionalContext :: None , } } }
};
}
