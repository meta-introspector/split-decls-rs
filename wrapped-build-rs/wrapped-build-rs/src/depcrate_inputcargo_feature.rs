// Generated macro for cargo_feature (function)
macro_rules! Depcrate_inputcargo_feature {
() => {
// Module: crate::input
// Provides: {"cargo_feature"}
// Dependencies: {}
# [doc = " For each activated feature of the package being built, this will be `true`."] # [track_caller] pub fn cargo_feature (name : & str) -> bool { if ! is_feature_name (name) { panic ! ("invalid feature name {name:?}") } let name = name . to_uppercase () . replace ('-' , "_") ; let key = format ! ("CARGO_FEATURE_{name}") ; ENV . is_present (& key) }
};
}
