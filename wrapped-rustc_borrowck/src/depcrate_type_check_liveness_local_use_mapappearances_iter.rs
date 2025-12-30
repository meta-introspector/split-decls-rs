// Generated macro for appearances_iter (function)
macro_rules! Depcrate_type_check_liveness_local_use_mapappearances_iter {
() => {
// Module: crate::type_check::liveness::local_use_map
// Provides: {"appearances_iter"}
// Dependencies: {}
fn appearances_iter (first : Option < AppearanceIndex > , appearances : & Appearances ,) -> impl Iterator < Item = AppearanceIndex > { AppearancesIter { appearances , current : first } }
};
}
