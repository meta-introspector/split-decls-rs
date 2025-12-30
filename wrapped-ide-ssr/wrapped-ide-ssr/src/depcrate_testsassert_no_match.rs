// Generated macro for assert_no_match (function)
macro_rules! Depcrate_testsassert_no_match {
() => {
// Module: crate::tests
// Provides: {"assert_no_match"}
// Dependencies: {}
fn assert_no_match (pattern : & str , code : & str) { let (db , position , selections) = single_file (code) ; hir :: attach_db (& db , | | { let mut match_finder = MatchFinder :: in_context (& db , ide_db :: FilePosition { file_id : position . file_id . file_id (& db) , offset : position . offset , } , selections . into_iter () . map (| selection | ide_db :: FileRange { file_id : selection . file_id . file_id (& db) , range : selection . range , }) . collect () ,) . unwrap () ; match_finder . add_search_pattern (pattern . parse () . unwrap ()) . unwrap () ; let matches = match_finder . matches () . flattened () . matches ; if ! matches . is_empty () { print_match_debug_info (& match_finder , position . file_id , & matches [0] . matched_text ()) ; panic ! ("Got {} matches when we expected none: {matches:#?}" , matches . len ()) ; } }) ; }
};
}
