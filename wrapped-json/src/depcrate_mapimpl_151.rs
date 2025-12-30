// Generated macro for impl_151 (impl)
macro_rules! Depcrate_mapimpl_151 {
() => {
// Module: crate::map
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Map < String , Value > { type Item = (& 'a String , & 'a Value) ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { Iter { iter : self . map . iter () , } } }
};
}
