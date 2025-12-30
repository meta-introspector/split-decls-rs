// Generated macro for Nullability (enum)
macro_rules! Depcrate_configNullability {
() => {
// Module: crate::config
// Provides: {"Nullability"}
// Dependencies: {}
# [derive (Deserialize , Debug , Clone , Copy , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub enum Nullability { # [serde (rename = "nullable")] Nullable , # [serde (rename = "nonnull")] NonNull , }
};
}
