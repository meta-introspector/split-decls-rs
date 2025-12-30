// Generated macro for impl_319 (impl)
macro_rules! Depcrate_mapimpl_319 {
() => {
// Module: crate::map
// Provides: {"impl_319"}
// Dependencies: {}
impl < K , V > IterMut < '_ , K , V > { # [doc = " Returns a iterator of references over the remaining items."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
