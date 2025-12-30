// Generated macro for impl_26 (impl)
macro_rules! Depcrate_dictionaryimpl_26 {
() => {
// Module: crate::dictionary
// Provides: {"impl_26"}
// Dependencies: {}
impl IntoIterator for Dictionary { type Item = (String , Value) ; type IntoIter = IntoIter ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { iter : self . map . into_iter () , } } }
};
}
