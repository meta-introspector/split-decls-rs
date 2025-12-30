// Generated macro for get_possible_values (function)
macro_rules! Depcrate_renderget_possible_values {
() => {
// Module: crate::render
// Provides: {"get_possible_values"}
// Dependencies: {}
fn get_possible_values (arg : & Arg) -> Option < (Vec < String > , bool) > { if arg . is_hide_possible_values_set () { return None ; } let possibles = & arg . get_possible_values () ; let possibles : Vec < & clap :: builder :: PossibleValue > = possibles . iter () . filter (| pos | ! pos . is_hide_set ()) . collect () ; if ! possibles . is_empty () { return Some (format_possible_values (& possibles)) ; } None }
};
}
