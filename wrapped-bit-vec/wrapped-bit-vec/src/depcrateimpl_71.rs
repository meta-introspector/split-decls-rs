// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < B : BitBlock > Iterator for Blocks < '_ , B > { type Item = B ; # [inline] fn next (& mut self) -> Option < B > { self . iter . next () . cloned () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
