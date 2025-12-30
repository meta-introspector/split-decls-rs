// Generated macro for impl_18 (impl)
macro_rules! Depcrate_core_dir_entry_iterimpl_18 {
() => {
// Module: crate::core::dir_entry_iter
// Provides: {"impl_18"}
// Dependencies: {}
impl < C : ClientState > Iterator for DirEntryIter < C > { type Item = Result < DirEntry < C > > ; fn next (& mut self) -> Option < Self :: Item > { loop { let top_read_dir_results = self . read_dir_results_stack . last_mut () ? ; if let Some (dir_entry_result) = top_read_dir_results . next () { let mut dir_entry = match dir_entry_result { Ok (dir_entry) => dir_entry , Err (err) => return Some (Err (err)) , } ; if dir_entry . read_children_path . is_some () { let iter = match self . read_dir_iter . as_mut () . ok_or_else (Error :: busy) { Ok (iter) => iter , Err (err) => return Some (Err (err)) , } ; if let Err (err) = Self :: push_next_read_dir_results (iter , & mut self . read_dir_results_stack) { dir_entry . read_children_error = Some (err) ; } } if dir_entry . depth >= self . min_depth { return Some (Ok (dir_entry)) ; } } else { self . read_dir_results_stack . pop () ; } } } }
};
}
