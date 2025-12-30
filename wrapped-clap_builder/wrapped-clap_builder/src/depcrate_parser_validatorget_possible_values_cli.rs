// Generated macro for get_possible_values_cli (function)
macro_rules! Depcrate_parser_validatorget_possible_values_cli {
() => {
// Module: crate::parser::validator
// Provides: {"get_possible_values_cli"}
// Dependencies: {}
pub (crate) fn get_possible_values_cli (a : & Arg) -> Vec < PossibleValue > { if ! a . is_takes_value_set () { vec ! [] } else { a . get_value_parser () . possible_values () . map (| pvs | pvs . collect ()) . unwrap_or_default () } }
};
}
