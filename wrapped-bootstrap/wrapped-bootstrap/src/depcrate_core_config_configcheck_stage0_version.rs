// Generated macro for check_stage0_version (function)
macro_rules! Depcrate_core_config_configcheck_stage0_version {
() => {
// Module: crate::core::config::config
// Provides: {"check_stage0_version"}
// Dependencies: {}
# [doc = " check rustc/cargo version is same or lower with 1 apart from the building one"] # [cfg (not (test))] pub fn check_stage0_version (program_path : & Path , component_name : & 'static str , src_dir : & Path , exec_ctx : & ExecutionContext ,) { use build_helper :: util :: fail ; if exec_ctx . dry_run () { return ; } let stage0_output = command (program_path) . arg ("--version") . run_capture_stdout (exec_ctx) . stdout () ; let mut stage0_output = stage0_output . lines () . next () . unwrap () . split (' ') ; let stage0_name = stage0_output . next () . unwrap () ; if stage0_name != component_name { fail (& format ! ("Expected to find {component_name} at {} but it claims to be {stage0_name}" , program_path . display ())) ; } let stage0_version = semver :: Version :: parse (stage0_output . next () . unwrap () . split ('-') . next () . unwrap () . trim ()) . unwrap () ; let source_version = semver :: Version :: parse (fs :: read_to_string (src_dir . join ("src/version")) . unwrap () . trim ()) . unwrap () ; if ! (source_version == stage0_version || (source_version . major == stage0_version . major && (source_version . minor == stage0_version . minor || source_version . minor == stage0_version . minor + 1))) { let prev_version = format ! ("{}.{}.x" , source_version . major , source_version . minor - 1) ; fail (& format ! ("Unexpected {component_name} version: {stage0_version}, we should use {prev_version}/{source_version} to build source with {source_version}")) ; } }
};
}
