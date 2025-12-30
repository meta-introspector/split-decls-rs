// Generated macro for impl_250 (impl)
macro_rules! Depcrate_searchimpl_250 {
() => {
// Module: crate::search
// Provides: {"impl_250"}
// Dependencies: {}
impl UsageSearchResult { pub fn is_empty (& self) -> bool { self . references . is_empty () } pub fn len (& self) -> usize { self . references . len () } pub fn iter (& self) -> impl Iterator < Item = (EditionedFileId , & [FileReference]) > + '_ { self . references . iter () . map (| (& file_id , refs) | (file_id , & * * refs)) } pub fn file_ranges (& self) -> impl Iterator < Item = FileRange > + '_ { self . references . iter () . flat_map (| (& file_id , refs) | { refs . iter () . map (move | & FileReference { range , .. } | FileRange { file_id , range }) }) } }
};
}
