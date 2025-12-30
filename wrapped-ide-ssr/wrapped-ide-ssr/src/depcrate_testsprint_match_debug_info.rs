// Generated macro for print_match_debug_info (function)
macro_rules! Depcrate_testsprint_match_debug_info {
() => {
// Module: crate::tests
// Provides: {"print_match_debug_info"}
// Dependencies: {}
# [allow (clippy :: print_stdout)] fn print_match_debug_info (match_finder : & MatchFinder < '_ > , file_id : EditionedFileId , snippet : & str) { let debug_info = match_finder . debug_where_text_equal (file_id , snippet) ; println ! ("Match debug info: {} nodes had text exactly equal to '{}'" , debug_info . len () , snippet) ; for (index , d) in debug_info . iter () . enumerate () { println ! ("Node #{index}\n{d:#?}\n") ; } }
};
}
