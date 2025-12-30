// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a > Iterator for Union < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
