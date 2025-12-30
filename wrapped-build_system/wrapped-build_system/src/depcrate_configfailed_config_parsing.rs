// Generated macro for failed_config_parsing (function)
macro_rules! Depcrate_configfailed_config_parsing {
() => {
// Module: crate::config
// Provides: {"failed_config_parsing"}
// Dependencies: {}
fn failed_config_parsing (config_file : & Path , err : & str) -> Result < ConfigFile , String > { Err (format ! ("Failed to parse `{}`: {}" , config_file . display () , err)) }
};
}
