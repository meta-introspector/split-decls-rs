// Generated macro for skip_host_target_lib (function)
macro_rules! Depcrate_core_build_steps_distskip_host_target_lib {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"skip_host_target_lib"}
// Dependencies: {}
fn skip_host_target_lib (builder : & Builder < '_ > , compiler : Compiler) -> bool { if ! builder . config . is_host_target (compiler . host) { builder . info ("\tskipping, not a build host") ; true } else { false } }
};
}
