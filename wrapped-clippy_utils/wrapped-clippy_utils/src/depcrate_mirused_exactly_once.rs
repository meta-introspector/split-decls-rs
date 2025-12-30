// Generated macro for used_exactly_once (function)
macro_rules! Depcrate_mirused_exactly_once {
() => {
// Module: crate::mir
// Provides: {"used_exactly_once"}
// Dependencies: {}
# [doc = " Convenience wrapper around `visit_local_usage`."] pub fn used_exactly_once (mir : & Body < '_ > , local : Local) -> Option < bool > { visit_local_usage (& [local] , mir , Location { block : START_BLOCK , statement_index : 0 , } ,) . map (| mut vec | { let LocalUsage { local_use_locs , .. } = vec . remove (0) ; let mut locations = local_use_locs . into_iter () . filter (| & location | ! is_local_assignment (mir , local , location)) ; if let Some (location) = locations . next () { locations . next () . is_none () && ! block_in_cycle (mir , location . block) } else { false } }) }
};
}
