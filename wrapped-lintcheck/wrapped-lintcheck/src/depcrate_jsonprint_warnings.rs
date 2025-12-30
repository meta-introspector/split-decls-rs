// Generated macro for print_warnings (function)
macro_rules! Depcrate_jsonprint_warnings {
() => {
// Module: crate::json
// Provides: {"print_warnings"}
// Dependencies: {}
# [doc = " Prints a section of warnings with a header and formatted code blocks."] fn print_warnings (title : & str , warnings : & [LintJson] , truncate_after : usize) { if warnings . is_empty () { return ; } print_h3 (& warnings [0] . name , title) ; println ! () ; let warnings = truncate (warnings , truncate_after) ; for warning in warnings { println ! ("{}" , warning . info_text (title)) ; println ! () ; println ! ("```") ; println ! ("{}" , warning . rendered) ; println ! ("```") ; println ! () ; } }
};
}
