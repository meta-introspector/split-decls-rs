// Generated macro for impl_27 (impl)
macro_rules! Depcrate_cacheimpl_27 {
() => {
// Module: crate::cache
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , I , R > IntoIterator for & 'a Cache < I , R > where I : Iterator , { type Item = & 'a I :: Item ; type IntoIter = CacheIter < 'a , I , R > ; fn into_iter (self) -> Self :: IntoIter { CacheIter { cache : self , curr : 0 , } } }
};
}
