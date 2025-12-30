// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dictionaryimpl_23 {
() => {
// Module: crate::dictionary
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Dictionary { type Item = (& 'a String , & 'a mut Value) ; type IntoIter = IterMut < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IterMut { iter : self . map . iter_mut () , } } }
};
}
