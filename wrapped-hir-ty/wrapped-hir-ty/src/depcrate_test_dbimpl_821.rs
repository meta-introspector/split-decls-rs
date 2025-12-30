// Generated macro for impl_821 (impl)
macro_rules! Depcrate_test_dbimpl_821 {
() => {
// Module: crate::test_db
// Provides: {"impl_821"}
// Dependencies: {}
impl TestDB { pub (crate) fn module_for_file_opt (& self , file_id : impl Into < FileId >) -> Option < ModuleId > { let file_id = file_id . into () ; for & krate in self . relevant_crates (file_id) . iter () { let crate_def_map = crate_def_map (self , krate) ; for (local_id , data) in crate_def_map . modules () { if data . origin . file_id () . map (| file_id | file_id . file_id (self)) == Some (file_id) { return Some (crate_def_map . module_id (local_id)) ; } } } None } pub (crate) fn module_for_file (& self , file_id : impl Into < FileId >) -> ModuleId { self . module_for_file_opt (file_id . into ()) . unwrap () } pub (crate) fn extract_annotations (& self ,) -> FxHashMap < EditionedFileId , Vec < (TextRange , String) > > { let mut files = Vec :: new () ; for & krate in self . all_crates () . iter () { let crate_def_map = crate_def_map (self , krate) ; for (module_id , _) in crate_def_map . modules () { let file_id = crate_def_map [module_id] . origin . file_id () ; files . extend (file_id) } } files . into_iter () . filter_map (| file_id | { let text = self . file_text (file_id . file_id (self)) ; let annotations = extract_annotations (text . text (self)) ; if annotations . is_empty () { return None ; } Some ((file_id , annotations)) }) . collect () } }
};
}
