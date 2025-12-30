// Generated macro for CargoFeatures (enum)
macro_rules! Depcrate_cargo_workspaceCargoFeatures {
() => {
// Module: crate::cargo_workspace
// Provides: {"CargoFeatures"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub enum CargoFeatures { All , Selected { # [doc = " List of features to activate."] features : Vec < String > , # [doc = " Do not activate the `default` feature."] no_default_features : bool , } , }
};
}
