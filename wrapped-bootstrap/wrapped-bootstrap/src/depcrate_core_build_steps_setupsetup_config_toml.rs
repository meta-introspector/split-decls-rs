// Generated macro for setup_config_toml (function)
macro_rules! Depcrate_core_build_steps_setupsetup_config_toml {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"setup_config_toml"}
// Dependencies: {}
fn setup_config_toml (path : & Path , profile : Profile , config : & Config) { if profile == Profile :: None { return ; } let latest_change_id = CONFIG_CHANGE_HISTORY . last () . unwrap () . change_id ; let settings = format ! ("# See bootstrap.example.toml for documentation of available options\n\
    #\n\
    profile = \"{profile}\"  # Includes one of the default files in {PROFILE_DIR}\n\
    change-id = {latest_change_id}\n") ; t ! (fs :: write (path , settings)) ; let include_path = profile . include_path (& config . src) ; println ! ("`x.py` will now use the configuration at {}" , include_path . display ()) ; }
};
}
