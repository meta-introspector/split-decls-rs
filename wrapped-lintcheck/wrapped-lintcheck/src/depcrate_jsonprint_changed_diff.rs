// Generated macro for print_changed_diff (function)
macro_rules! Depcrate_jsonprint_changed_diff {
() => {
// Module: crate::json
// Provides: {"print_changed_diff"}
// Dependencies: {}
# [doc = " Prints a section of changed warnings with unified diff format."] fn print_changed_diff (changed : & [(LintJson , LintJson)] , truncate_after : usize) { if changed . is_empty () { return ; } print_h3 (& changed [0] . 0 . name , "Changed") ; println ! () ; let changed = truncate (changed , truncate_after) ; for (old , new) in changed { println ! ("{}" , new . info_text ("Changed")) ; println ! () ; println ! ("```diff") ; for change in diff :: lines (& old . rendered , & new . rendered) { use diff :: Result :: { Both , Left , Right } ; match change { Both (unchanged , _) => { println ! (" {unchanged}") ; } , Left (removed) => { println ! ("-{removed}") ; } , Right (added) => { println ! ("+{added}") ; } , } } println ! ("```") ; } }
};
}
