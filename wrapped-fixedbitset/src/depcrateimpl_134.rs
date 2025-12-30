// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > Iterator for SymmetricDifference < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
