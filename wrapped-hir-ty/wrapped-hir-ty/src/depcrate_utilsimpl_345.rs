// Generated macro for impl_345 (impl)
macro_rules! Depcrate_utilsimpl_345 {
() => {
// Module: crate::utils
// Provides: {"impl_345"}
// Dependencies: {}
impl Iterator for SuperTraits < '_ > { type Item = TraitRef ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . stack . pop () { self . elaborate (& next) ; Some (next) } else { None } } }
};
}
