// Generated macro for checkout_toolstate_repo (function)
macro_rules! Depcrate_core_build_steps_toolstatecheckout_toolstate_repo {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"checkout_toolstate_repo"}
// Dependencies: {}
# [doc = " Checks out the toolstate repo into `TOOLSTATE_DIR`."] fn checkout_toolstate_repo (builder : & Builder < '_ >) { if let Ok (token) = env :: var ("TOOLSTATE_REPO_ACCESS_TOKEN") { prepare_toolstate_config (builder , & token) ; } if Path :: new (TOOLSTATE_DIR) . exists () { eprintln ! ("Cleaning old toolstate directory...") ; t ! (fs :: remove_dir_all (TOOLSTATE_DIR)) ; } helpers :: git (None) . arg ("clone") . arg ("--depth=1") . arg (toolstate_repo ()) . arg (TOOLSTATE_DIR) . run (builder) ; }
};
}
