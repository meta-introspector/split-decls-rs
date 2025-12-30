// Generated macro for impl_13 (impl)
macro_rules! Depcrate_appimpl_13 {
() => {
// Module: crate::app
// Provides: {"impl_13"}
// Dependencies: {}
impl Iterator for RandomSignal { type Item = u64 ; fn next (& mut self) -> Option < u64 > { Some (self . distribution . sample (& mut self . rng)) } }
};
}
