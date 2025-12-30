// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < B : BitBlock > Iterator for Difference < '_ , B > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . count () } }
};
}
