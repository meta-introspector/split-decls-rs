// Generated macro for config_name (module)
macro_rules! Depcrate_parse_nom_testsconfig_name {
() => {
// Module: crate::parse::nom::tests
// Provides: {"config_name"}
// Dependencies: {}
mod config_name { use winnow :: prelude :: * ; use super :: config_name ; use crate :: parse :: tests :: util :: fully_consumed ; # [test] fn just_name () { assert_eq ! (config_name . parse_peek (b"name") . unwrap () , fully_consumed ("name" . into ())) ; } # [test] fn must_start_with_alphabetic () { assert ! (config_name . parse_peek (b"4aaa") . is_err ()) ; assert ! (config_name . parse_peek (b"-aaa") . is_err ()) ; } # [test] fn only_a_subset_of_characters_is_allowed () { assert ! (config_name . parse (b"Name$_") . is_err ()) ; assert ! (config_name . parse (b"other#") . is_err ()) ; } # [test] fn cannot_be_empty () { assert ! (config_name . parse_peek (b"") . is_err ()) ; } }
};
}
