// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut LruSlab < T > { type Item = (u32 , & 'a mut T) ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
