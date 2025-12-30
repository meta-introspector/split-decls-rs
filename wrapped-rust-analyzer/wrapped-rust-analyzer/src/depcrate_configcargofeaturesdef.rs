// Generated macro for CargoFeaturesDef (enum)
macro_rules! Depcrate_configCargoFeaturesDef {
() => {
// Module: crate::config
// Provides: {"CargoFeaturesDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum CargoFeaturesDef { All , # [serde (untagged)] Selected (Vec < String >) , }
};
}
