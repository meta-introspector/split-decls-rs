// Generated macro for strip_debug (function)
macro_rules! Depcrate_core_build_steps_compilestrip_debug {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"strip_debug"}
// Dependencies: {}
pub fn strip_debug (builder : & Builder < '_ > , target : TargetSelection , path : & Path) { if target != "x86_64-unknown-linux-gnu" || ! builder . config . is_host_target (target) || ! path . exists () { return ; } let previous_mtime = t ! (t ! (path . metadata ()) . modified ()) ; let stamp = BuildStamp :: new (path . parent () . unwrap ()) . with_prefix (path . file_name () . unwrap () . to_str () . unwrap ()) . with_prefix ("strip") . add_stamp (previous_mtime . duration_since (SystemTime :: UNIX_EPOCH) . unwrap () . as_nanos ()) ; if ! stamp . is_up_to_date () { command ("strip") . arg ("--strip-debug") . arg (path) . run_capture (builder) ; } t ! (stamp . write ()) ; let file = t ! (fs :: File :: open (path)) ; t ! (file . set_modified (previous_mtime)) ; }
};
}
