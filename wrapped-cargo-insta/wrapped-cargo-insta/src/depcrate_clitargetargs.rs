// Generated macro for TargetArgs (struct)
macro_rules! Depcrate_cliTargetArgs {
() => {
// Module: crate::cli
// Provides: {"TargetArgs"}
// Dependencies: {}
# [derive (Args , Debug , Clone)] struct TargetArgs { # [doc = " Path to `Cargo.toml`"] # [arg (long , value_name = "PATH")] manifest_path : Option < PathBuf > , # [doc = " Explicit path to the workspace root"] # [arg (long , value_name = "PATH")] workspace_root : Option < PathBuf > , # [doc = " Sets the extensions to consider. Defaults to `snap`."] # [arg (short = 'e' , long , value_name = "EXTENSIONS" , num_args = 1 .., value_delimiter = ',' , default_value = "snap")] extensions : Vec < String > , # [doc = " Work on all packages in the workspace"] # [arg (long)] workspace : bool , # [doc = " Alias for `--workspace` (deprecated)"] # [arg (long)] all : bool , # [doc = " Also walk into ignored paths."] # [arg (long , alias = "no-ignore")] include_ignored : bool , # [doc = " Also include hidden paths."] # [arg (long)] include_hidden : bool , }
};
}
