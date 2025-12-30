// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < B : BitBlock > Iterator for Union < '_ , B > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . count () } }
};
}
