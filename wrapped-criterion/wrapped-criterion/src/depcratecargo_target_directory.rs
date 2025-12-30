// Generated macro for cargo_target_directory (function)
macro_rules! Depcratecargo_target_directory {
() => {
// Module: crate
// Provides: {"cargo_target_directory"}
// Dependencies: {}
# [doc = " Returns the Cargo target directory, possibly calling `cargo metadata` to"] # [doc = " figure it out."] fn cargo_target_directory () -> Option < PathBuf > { # [derive (Deserialize)] struct Metadata { target_directory : PathBuf , } env :: var_os ("CARGO_TARGET_DIR") . map (PathBuf :: from) . or_else (| | { let output = Command :: new (env :: var_os ("CARGO") ?) . args (["metadata" , "--format-version" , "1"]) . output () . ok () ? ; let metadata : Metadata = serde_json :: from_slice (& output . stdout) . ok () ? ; Some (metadata . target_directory) }) }
};
}
