// Generated macro for tests (module)
macro_rules! Depcrate_matchingtests {
() => {
// Module: crate::matching
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { MatchFinder , SsrRule } ; # [test] fn parse_match_replace () { let rule : SsrRule = "foo($x) ==>> bar($x)" . parse () . unwrap () ; let input = "fn foo() {} fn bar() {} fn main() { foo(1+2); }" ; let (db , position , selections) = crate :: tests :: single_file (input) ; hir :: attach_db (& db , | | { let position = ide_db :: FilePosition { file_id : position . file_id . file_id (& db) , offset : position . offset , } ; let mut match_finder = MatchFinder :: in_context (& db , position , selections . into_iter () . map (| frange | ide_db :: FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range , }) . collect () ,) . unwrap () ; match_finder . add_rule (rule) . unwrap () ; let matches = match_finder . matches () ; assert_eq ! (matches . matches . len () , 1) ; assert_eq ! (matches . matches [0] . matched_node . text () , "foo(1+2)") ; assert_eq ! (matches . matches [0] . placeholder_values . len () , 1) ; let edits = match_finder . edits () ; assert_eq ! (edits . len () , 1) ; let edit = & edits [& position . file_id] ; let mut after = input . to_owned () ; edit . apply (& mut after) ; assert_eq ! (after , "fn foo() {} fn bar() {} fn main() { bar(1+2); }") ; }) ; } }
};
}
