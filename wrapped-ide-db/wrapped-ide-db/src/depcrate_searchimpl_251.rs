// Generated macro for impl_251 (impl)
macro_rules! Depcrate_searchimpl_251 {
() => {
// Module: crate::search
// Provides: {"impl_251"}
// Dependencies: {}
impl IntoIterator for UsageSearchResult { type Item = (EditionedFileId , Vec < FileReference >) ; type IntoIter = < FxHashMap < EditionedFileId , Vec < FileReference > > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . references . into_iter () } }
};
}
