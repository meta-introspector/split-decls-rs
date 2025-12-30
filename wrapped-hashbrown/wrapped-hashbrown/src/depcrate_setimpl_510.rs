// Generated macro for impl_510 (impl)
macro_rules! Depcrate_setimpl_510 {
() => {
// Module: crate::set
// Provides: {"impl_510"}
// Dependencies: {}
impl < 'a , T , S , A : Allocator > IntoIterator for & 'a HashSet < T , S , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
