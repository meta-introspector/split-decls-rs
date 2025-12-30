// Generated macro for impl_341 (impl)
macro_rules! Depcrate_mapimpl_341 {
() => {
// Module: crate::map
// Provides: {"impl_341"}
// Dependencies: {}
impl < K , V , A : Allocator > Drain < '_ , K , V , A > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . iter () , marker : PhantomData , } } }
};
}
