// Generated macro for impl_119 (impl)
macro_rules! Depcrate_rawimpl_119 {
() => {
// Module: crate::raw
// Provides: {"impl_119"}
// Dependencies: {}
impl < T , A : Allocator > Iterator for RawIntoIter < T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < T > { unsafe { Some (self . iter . next () ? . read ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
