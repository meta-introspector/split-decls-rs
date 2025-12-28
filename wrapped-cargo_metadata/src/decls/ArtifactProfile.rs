macro_rules! deps {
    () => {
        ArtifactDebuginfo!();
    };
}

macro_rules! ArtifactProfile {
    () => {
        deps!();
        # [doc = " Profile settings used to determine which compiler flags to use for a"] # [doc = " target."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct ArtifactProfile { # [doc = " Optimization level. Possible values are 0-3, s or z."] pub opt_level : String , # [doc = " The kind of debug information."] # [serde (default)] pub debuginfo : ArtifactDebuginfo , # [doc = " State of the `cfg(debug_assertions)` directive, enabling macros like"] # [doc = " `debug_assert!`"] pub debug_assertions : bool , # [doc = " State of the overflow checks."] pub overflow_checks : bool , # [doc = " Whether this profile is a test"] pub test : bool , }
    };
}

ArtifactProfile!();