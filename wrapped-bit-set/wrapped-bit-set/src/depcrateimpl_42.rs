// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < B : BitBlock > Iterator for SymmetricDifference < '_ , B > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . count () } }
};
}
