// Generated macro for impl_19 (impl)
macro_rules! Depcrate_dictionaryimpl_19 {
() => {
// Module: crate::dictionary
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Dictionary { type Item = (& 'a String , & 'a Value) ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { Iter { iter : self . map . iter () , } } }
};
}
