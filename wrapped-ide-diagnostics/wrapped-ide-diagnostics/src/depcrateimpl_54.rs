// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl LintGroups { fn contains (& self , group : & str) -> bool { self . groups . contains (& group) || (self . inside_warnings && group == "warnings") } }
};
}
