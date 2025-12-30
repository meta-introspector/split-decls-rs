// Generated macro for CargoConfig (struct)
macro_rules! Depcrate_cargo_workspaceCargoConfig {
() => {
// Module: crate::cargo_workspace
// Provides: {"CargoConfig"}
// Dependencies: {}
# [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct CargoConfig { # [doc = " Whether to pass `--all-targets` to cargo invocations."] pub all_targets : bool , # [doc = " List of features to activate."] pub features : CargoFeatures , # [doc = " rustc target"] pub target : Option < String > , # [doc = " Sysroot loading behavior"] pub sysroot : Option < RustLibSource > , pub sysroot_src : Option < AbsPathBuf > , # [doc = " rustc private crate source"] pub rustc_source : Option < RustLibSource > , # [doc = " Extra includes to add to the VFS."] pub extra_includes : Vec < AbsPathBuf > , pub cfg_overrides : CfgOverrides , # [doc = " Invoke `cargo check` through the RUSTC_WRAPPER."] pub wrap_rustc_in_build_scripts : bool , # [doc = " The command to run instead of `cargo check` for building build scripts."] pub run_build_script_command : Option < Vec < String > > , # [doc = " Extra args to pass to the cargo command."] pub extra_args : Vec < String > , # [doc = " Extra env vars to set when invoking the cargo command"] pub extra_env : FxHashMap < String , Option < String > > , pub invocation_strategy : InvocationStrategy , # [doc = " Optional path to use instead of `target` when building"] pub target_dir_config : TargetDirectoryConfig , # [doc = " Gate `#[test]` behind `#[cfg(test)]`"] pub set_test : bool , # [doc = " Load the project without any dependencies"] pub no_deps : bool , }
};
}
