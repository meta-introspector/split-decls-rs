// Generated macro for impl_127 (impl)
macro_rules! Depcrate_rawimpl_127 {
() => {
// Module: crate::raw
// Provides: {"impl_127"}
// Dependencies: {}
impl < T , A : Allocator > Iterator for RawDrain < '_ , T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < T > { unsafe { let item = self . iter . next () ? ; Some (item . read ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
