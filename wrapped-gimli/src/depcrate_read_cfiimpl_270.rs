// Generated macro for impl_270 (impl)
macro_rules! Depcrate_read_cfiimpl_270 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'iter , T : ReaderOffset > Iterator for RegisterRuleIter < 'iter , T > { type Item = & 'iter (Register , RegisterRule < T >) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } }
};
}
