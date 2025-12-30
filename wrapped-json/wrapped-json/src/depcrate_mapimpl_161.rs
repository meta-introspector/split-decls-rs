// Generated macro for impl_161 (impl)
macro_rules! Depcrate_mapimpl_161 {
() => {
// Module: crate::map
// Provides: {"impl_161"}
// Dependencies: {}
impl IntoIterator for Map < String , Value > { type Item = (String , Value) ; type IntoIter = IntoIter ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { iter : self . map . into_iter () , } } }
};
}
