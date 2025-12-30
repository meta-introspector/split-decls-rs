// Generated macro for read_old_toolstate (function)
macro_rules! Depcrate_core_build_steps_toolstateread_old_toolstate {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"read_old_toolstate"}
// Dependencies: {}
# [doc = " Reads the latest toolstate from the toolstate repo."] fn read_old_toolstate () -> Vec < RepoState > { let latest_path = Path :: new (TOOLSTATE_DIR) . join ("_data") . join ("latest.json") ; let old_toolstate = t ! (fs :: read (latest_path)) ; t ! (serde_json :: from_slice (& old_toolstate)) }
};
}
