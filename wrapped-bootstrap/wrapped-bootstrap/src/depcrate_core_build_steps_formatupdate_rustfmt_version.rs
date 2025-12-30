// Generated macro for update_rustfmt_version (function)
macro_rules! Depcrate_core_build_steps_formatupdate_rustfmt_version {
() => {
// Module: crate::core::build_steps::format
// Provides: {"update_rustfmt_version"}
// Dependencies: {}
# [doc = " Updates the last rustfmt version used."] fn update_rustfmt_version (build : & Builder < '_ >) { let Some ((version , stamp_file)) = get_rustfmt_version (build) else { return ; } ; t ! (stamp_file . add_stamp (version) . write ()) ; }
};
}
