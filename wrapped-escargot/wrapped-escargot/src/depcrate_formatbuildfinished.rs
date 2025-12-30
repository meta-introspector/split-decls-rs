// Generated macro for BuildFinished (struct)
macro_rules! Depcrate_formatBuildFinished {
() => {
// Module: crate::format
// Provides: {"BuildFinished"}
// Dependencies: {}
# [doc = " Build completed, all further output should not be parsed."] # [doc = ""] # [doc = " See <https://doc.rust-lang.org/cargo/reference/external-tools.html#build-finished>"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct BuildFinished { # [doc = " Whether or not the build finished successfully."] pub success : bool , }
};
}
