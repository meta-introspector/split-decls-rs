// Generated macro for impl_321 (impl)
macro_rules! Depcrate_mapimpl_321 {
() => {
// Module: crate::map
// Provides: {"impl_321"}
// Dependencies: {}
impl < K , V , A : Allocator > IntoIter < K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . iter () , marker : PhantomData , } } }
};
}
