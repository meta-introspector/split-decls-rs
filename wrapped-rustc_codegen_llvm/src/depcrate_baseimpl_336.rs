// Generated macro for impl_336 (impl)
macro_rules! Depcrate_baseimpl_336 {
() => {
// Module: crate::base
// Provides: {"impl_336"}
// Dependencies: {}
impl < 'll > Iterator for ValueIter < 'll > { type Item = & 'll Value ; fn next (& mut self) -> Option < & 'll Value > { let old = self . cur ; if let Some (old) = old { self . cur = unsafe { (self . step) (old) } ; } old } }
};
}
