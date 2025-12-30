// Generated macro for parse_seed_line (function)
macro_rules! Depcrate_test_runner_failure_persistence_fileparse_seed_line {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"parse_seed_line"}
// Dependencies: {}
fn parse_seed_line (mut line : String , path : & Path , lineno : usize ,) -> Option < PersistedSeed > { if let Some (comment_start) = line . find ('#') { line . truncate (comment_start) ; } if line . len () > 0 { let ret = line . parse :: < PersistedSeed > () . ok () ; if ! ret . is_some () { eprintln ! ("proptest: {}:{}: unparsable line, ignoring" , path . display () , lineno + 1) ; } return ret ; } None }
};
}
