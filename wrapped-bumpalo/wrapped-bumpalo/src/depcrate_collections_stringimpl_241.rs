// Generated macro for impl_241 (impl)
macro_rules! Depcrate_collections_stringimpl_241 {
() => {
// Module: crate::collections::string
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'a , 'bump > Iterator for Drain < 'a , 'bump > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
