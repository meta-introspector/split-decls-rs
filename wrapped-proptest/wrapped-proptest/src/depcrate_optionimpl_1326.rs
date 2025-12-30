// Generated macro for impl_1326 (impl)
macro_rules! Depcrate_optionimpl_1326 {
() => {
// Module: crate::option
// Provides: {"impl_1326"}
// Dependencies: {}
impl < T : fmt :: Debug > ValueTree for NoneStrategy < T > { type Value = Option < T > ; fn current (& self) -> Option < T > { None } fn simplify (& mut self) -> bool { false } fn complicate (& mut self) -> bool { false } }
};
}
