// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl LintGroups { fn contains (& self , group : & str) -> bool { self . groups . contains (& group) || (self . inside_warnings && group == "warnings") } }
};
}
