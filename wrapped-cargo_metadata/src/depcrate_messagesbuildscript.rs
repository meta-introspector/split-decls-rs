// Generated macro for BuildScript (struct)
macro_rules! Depcrate_messagesBuildScript {
() => {
// Module: crate::messages
// Provides: {"BuildScript"}
// Dependencies: {}
# [doc = " Output of a build script execution."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct BuildScript { # [doc = " The package this build script execution belongs to"] pub package_id : PackageId , # [doc = " The libs to link"] pub linked_libs : Vec < Utf8PathBuf > , # [doc = " The paths to search when resolving libs"] pub linked_paths : Vec < Utf8PathBuf > , # [doc = " Various `--cfg` flags to pass to the compiler"] pub cfgs : Vec < String > , # [doc = " The environment variables to add to the compilation"] pub env : Vec < (String , String) > , # [doc = " The `OUT_DIR` environment variable where this script places its output"] # [doc = ""] # [doc = " Added in Rust 1.41."] # [serde (default)] pub out_dir : Utf8PathBuf , }
};
}
