// Generated macro for impl_558 (impl)
macro_rules! Depcrate_nfa_thompson_compilerimpl_558 {
() => {
// Module: crate::nfa::thompson::compiler
// Provides: {"impl_558"}
// Dependencies: {}
impl Utf8Node { fn set_last_transition (& mut self , next : StateID) { if let Some (last) = self . last . take () { self . trans . push (Transition { start : last . start , end : last . end , next , }) ; } } }
};
}
