// Generated macro for possible_values (function)
macro_rules! Depcrate_engine_completepossible_values {
() => {
// Module: crate::engine::complete
// Provides: {"possible_values"}
// Dependencies: {}
# [doc = " Get the possible values for completion"] fn possible_values (a : & clap :: Arg) -> Option < Vec < clap :: builder :: PossibleValue > > { if ! a . get_num_args () . expect ("built") . takes_values () { None } else { a . get_value_parser () . possible_values () . map (| pvs | pvs . collect ()) } }
};
}
