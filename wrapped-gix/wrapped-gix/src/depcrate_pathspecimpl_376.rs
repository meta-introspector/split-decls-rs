// Generated macro for impl_376 (impl)
macro_rules! Depcrate_pathspecimpl_376 {
() => {
// Module: crate::pathspec
// Provides: {"impl_376"}
// Dependencies: {}
# [doc = " Access"] impl PathspecDetached { # [doc = " Return the first [`Match`](search::Match) of `relative_path`, or `None`."] # [doc = " Note that the match might [be excluded](search::Match::is_excluded())."] # [doc = " `is_dir` is true if `relative_path` is a directory."] # [doc (alias = "match_diff" , alias = "match_tree" , alias = "match_index" , alias = "match_workdir" , alias = "matches_path" , alias = "git2")] pub fn pattern_matching_relative_path < 'a > (& mut self , relative_path : impl Into < & 'a BStr > , is_dir : Option < bool > ,) -> Option < gix_pathspec :: search :: Match < '_ > > { self . search . pattern_matching_relative_path (relative_path . into () , is_dir , & mut | relative_path , case , is_dir , out | { let stack = self . stack . as_mut () . expect ("initialized in advance") ; stack . set_case (case) . at_entry (relative_path , Some (is_dir_to_mode (is_dir)) , & self . odb) . is_ok_and (| platform | platform . matching_attributes (out)) } ,) } # [doc = " The simplified version of [`pattern_matching_relative_path()`](Self::pattern_matching_relative_path()) which returns"] # [doc = " `true` if `relative_path` is included in the set of positive pathspecs, while not being excluded."] pub fn is_included < 'a > (& mut self , relative_path : impl Into < & 'a BStr > , is_dir : Option < bool >) -> bool { self . pattern_matching_relative_path (relative_path , is_dir) . is_some_and (| m | ! m . is_excluded ()) } }
};
}
