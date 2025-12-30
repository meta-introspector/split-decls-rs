// Generated macro for impl_349 (impl)
macro_rules! Depcrate_utilsimpl_349 {
() => {
// Module: crate::utils
// Provides: {"impl_349"}
// Dependencies: {}
impl Iterator for ClauseElaborator < '_ > { type Item = WhereClause ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . stack . pop () { self . elaborate_supertrait (& next) ; Some (next) } else { None } } }
};
}
