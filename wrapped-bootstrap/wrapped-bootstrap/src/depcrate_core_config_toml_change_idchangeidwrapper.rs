// Generated macro for ChangeIdWrapper (struct)
macro_rules! Depcrate_core_config_toml_change_idChangeIdWrapper {
() => {
// Module: crate::core::config::toml::change_id
// Provides: {"ChangeIdWrapper"}
// Dependencies: {}
# [doc = " Since we use `#[serde(deny_unknown_fields)]` on `TomlConfig`, we need a wrapper type"] # [doc = " for the \"change-id\" field to parse it even if other fields are invalid. This ensures"] # [doc = " that if deserialization fails due to other fields, we can still provide the changelogs"] # [doc = " to allow developers to potentially find the reason for the failure in the logs.."] # [derive (Deserialize , Default)] pub (crate) struct ChangeIdWrapper { # [serde (alias = "change-id" , default , deserialize_with = "deserialize_change_id")] pub (crate) inner : Option < ChangeId > , }
};
}
