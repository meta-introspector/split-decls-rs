// Generated macro for construct_path_string (function)
macro_rules! Depcrate_upvarconstruct_path_string {
() => {
// Module: crate::upvar
// Provides: {"construct_path_string"}
// Dependencies: {}
fn construct_path_string < 'tcx > (tcx : TyCtxt < '_ > , place : & Place < 'tcx >) -> String { let place_str = construct_place_string (tcx , place) ; format ! ("{place_str} used here") }
};
}
