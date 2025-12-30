// Generated macro for is_layout_incompatible (function)
macro_rules! Depcrate_transmute_utilsis_layout_incompatible {
() => {
// Module: crate::transmute::utils
// Provides: {"is_layout_incompatible"}
// Dependencies: {}
pub (super) fn is_layout_incompatible < 'tcx > (cx : & LateContext < 'tcx > , from : Ty < 'tcx > , to : Ty < 'tcx >) -> bool { let typing_env = cx . typing_env () ; if let Ok (from) = cx . tcx . try_normalize_erasing_regions (typing_env , from) && let Ok (to) = cx . tcx . try_normalize_erasing_regions (typing_env , to) && let Ok (from_layout) = cx . tcx . layout_of (typing_env . as_query_input (from)) && let Ok (to_layout) = cx . tcx . layout_of (typing_env . as_query_input (to)) { from_layout . size != to_layout . size || from_layout . align . abi != to_layout . align . abi } else { false } }
};
}
