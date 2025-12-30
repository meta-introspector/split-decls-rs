// Generated macro for ArtifactProfile (struct)
macro_rules! Depcrate_formatArtifactProfile {
() => {
// Module: crate::format
// Provides: {"ArtifactProfile"}
// Dependencies: {}
# [doc = " Profile settings used to determine which compiler flags to use for a"] # [doc = " target."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct ArtifactProfile < 'a > { # [doc = " Optimization level. Possible values are 0-3, s or z."] # [serde (borrow)] pub opt_level : CowStr < 'a > , # [doc = " The amount of debug info."] pub debuginfo : Option < DebugInfo < 'a > > , # [doc = " State of the `cfg(debug_assertions)` directive, enabling macros like"] # [doc = " `debug_assert!`"] pub debug_assertions : bool , # [doc = " State of the overflow checks."] pub overflow_checks : bool , # [doc = " Whether this profile is a test"] pub test : bool , }
};
}
