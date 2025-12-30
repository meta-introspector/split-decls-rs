// Generated macro for LocationInfo (struct)
macro_rules! Depcrate_cliLocationInfo {
() => {
// Module: crate::cli
// Provides: {"LocationInfo"}
// Dependencies: {}
# [derive (Debug)] struct LocationInfo < 'a > { tool_config : ToolConfig , workspace_root : PathBuf , # [doc = " Packages to test"] packages : Vec < Package > , exts : Vec < & 'a str > , find_flags : FindFlags , # [doc = " The tested crate's insta version (i.e. not the `cargo-insta` binary"] # [doc = " that's running this code)."] insta_version : Version , }
};
}
