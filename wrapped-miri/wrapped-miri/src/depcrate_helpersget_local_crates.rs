// Generated macro for get_local_crates (function)
macro_rules! Depcrate_helpersget_local_crates {
() => {
// Module: crate::helpers
// Provides: {"get_local_crates"}
// Dependencies: {}
# [doc = " Retrieve the list of local crates that should have been passed by cargo-miri in"] # [doc = " MIRI_LOCAL_CRATES and turn them into `CrateNum`s."] pub fn get_local_crates (tcx : TyCtxt < '_ >) -> Vec < CrateNum > { let local_crate_names = std :: env :: var ("MIRI_LOCAL_CRATES") . map (| crates | crates . split (',') . map (| krate | krate . to_string ()) . collect :: < Vec < _ > > ()) . unwrap_or_default () ; let mut local_crates = Vec :: new () ; for & crate_num in tcx . crates (()) { let name = tcx . crate_name (crate_num) ; let name = name . as_str () ; if local_crate_names . iter () . any (| local_name | local_name == name) { local_crates . push (crate_num) ; } } local_crates }
};
}
