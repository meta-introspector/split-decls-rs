// Generated macro for impl_156 (impl)
macro_rules! Depcrate_mapimpl_156 {
() => {
// Module: crate::map
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Map < String , Value > { type Item = (& 'a String , & 'a mut Value) ; type IntoIter = IterMut < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IterMut { iter : self . map . iter_mut () , } } }
};
}
