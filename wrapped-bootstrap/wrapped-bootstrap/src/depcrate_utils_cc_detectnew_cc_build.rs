// Generated macro for new_cc_build (function)
macro_rules! Depcrate_utils_cc_detectnew_cc_build {
() => {
// Module: crate::utils::cc_detect
// Provides: {"new_cc_build"}
// Dependencies: {}
# [doc = " Creates and configures a new [`cc::Build`] instance for the given target."] fn new_cc_build (build : & Build , target : TargetSelection) -> cc :: Build { let mut cfg = cc :: Build :: new () ; cfg . cargo_metadata (false) . opt_level (2) . warnings (false) . debug (false) . flag_if_supported ("-gz") . target (& target . triple) . host (& build . host_target . triple) ; match build . crt_static (target) { Some (a) => { cfg . static_crt (a) ; } None => { if target . is_msvc () { cfg . static_crt (true) ; } if target . contains ("musl") { cfg . static_flag (true) ; } } } cfg }
};
}
