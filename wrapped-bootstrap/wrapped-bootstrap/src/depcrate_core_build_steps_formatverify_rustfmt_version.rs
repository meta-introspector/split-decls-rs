// Generated macro for verify_rustfmt_version (function)
macro_rules! Depcrate_core_build_steps_formatverify_rustfmt_version {
() => {
// Module: crate::core::build_steps::format
// Provides: {"verify_rustfmt_version"}
// Dependencies: {}
# [doc = " Return whether the format cache can be reused."] fn verify_rustfmt_version (build : & Builder < '_ >) -> bool { let Some ((version , stamp_file)) = get_rustfmt_version (build) else { return false ; } ; stamp_file . add_stamp (version) . is_up_to_date () }
};
}
