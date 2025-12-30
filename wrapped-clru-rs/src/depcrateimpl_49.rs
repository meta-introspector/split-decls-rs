// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < K , V , S > CLruCache < K , V , S > { # [doc = " Returns an iterator visiting all entries in order, giving a mutable reference on V."] # [doc = " The iterator element type is `(&'a K, &'a mut V)`."] pub fn iter_mut (& mut self) -> CLruCacheIterMut < '_ , K , V > { CLruCacheIterMut { iter : self . storage . iter_mut () , } } }
};
}
