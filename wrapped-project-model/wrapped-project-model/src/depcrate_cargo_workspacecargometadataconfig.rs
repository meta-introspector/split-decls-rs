// Generated macro for CargoMetadataConfig (struct)
macro_rules! Depcrate_cargo_workspaceCargoMetadataConfig {
() => {
// Module: crate::cargo_workspace
// Provides: {"CargoMetadataConfig"}
// Dependencies: {}
# [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct CargoMetadataConfig { # [doc = " List of features to activate."] pub features : CargoFeatures , # [doc = " rustc targets"] pub targets : Vec < String > , # [doc = " Extra args to pass to the cargo command."] pub extra_args : Vec < String > , # [doc = " Extra env vars to set when invoking the cargo command"] pub extra_env : FxHashMap < String , Option < String > > , # [doc = " What kind of metadata are we fetching: workspace, rustc, or sysroot."] pub kind : & 'static str , # [doc = " The toolchain version, if known."] # [doc = " Used to conditionally enable unstable cargo features."] pub toolchain_version : Option < semver :: Version > , }
};
}
