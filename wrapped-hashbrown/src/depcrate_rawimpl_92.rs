// Generated macro for impl_92 (impl)
macro_rules! Depcrate_rawimpl_92 {
() => {
// Module: crate::raw
// Provides: {"impl_92"}
// Dependencies: {}
impl < T , A : Allocator > IntoIterator for RawTable < T , A > { type Item = T ; type IntoIter = RawIntoIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> RawIntoIter < T , A > { unsafe { let iter = self . iter () ; self . into_iter_from (iter) } } }
};
}
