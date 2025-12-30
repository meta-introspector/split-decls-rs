// Generated macro for ChangeId (enum)
macro_rules! Depcrate_core_config_toml_change_idChangeId {
() => {
// Module: crate::core::config::toml::change_id
// Provides: {"ChangeId"}
// Dependencies: {}
# [doc = " This enum is used for deserializing change IDs from TOML, allowing both numeric values and the string `\"ignore\"`."] # [derive (Clone , Debug , PartialEq)] pub enum ChangeId { Ignore , Id (usize) , }
};
}
