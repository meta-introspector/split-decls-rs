// Generated macro for get_rustfmt_version (function)
macro_rules! Depcrate_core_build_steps_formatget_rustfmt_version {
() => {
// Module: crate::core::build_steps::format
// Provides: {"get_rustfmt_version"}
// Dependencies: {}
fn get_rustfmt_version (build : & Builder < '_ >) -> Option < (String , BuildStamp) > { let stamp_file = BuildStamp :: new (& build . out) . with_prefix ("rustfmt") ; let mut cmd = command (build . config . initial_rustfmt . as_ref () ?) ; cmd . arg ("--version") ; let output = cmd . allow_failure () . run_capture (build) ; if output . is_failure () { return None ; } Some ((output . stdout () , stamp_file)) }
};
}
