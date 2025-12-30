// Generated macro for impl_216 (impl)
macro_rules! Depcrate_searchimpl_216 {
() => {
// Module: crate::search
// Provides: {"impl_216"}
// Dependencies: {}
impl IntoIterator for UsageSearchResult { type Item = (EditionedFileId , Vec < FileReference >) ; type IntoIter = < FxHashMap < EditionedFileId , Vec < FileReference > > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . references . into_iter () } }
};
}
