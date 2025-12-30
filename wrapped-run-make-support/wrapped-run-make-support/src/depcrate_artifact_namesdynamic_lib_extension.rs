// Generated macro for dynamic_lib_extension (function)
macro_rules! Depcrate_artifact_namesdynamic_lib_extension {
() => {
// Module: crate::artifact_names
// Provides: {"dynamic_lib_extension"}
// Dependencies: {}
# [doc = " Construct the dynamic library extension based on the target."] # [must_use] pub fn dynamic_lib_extension () -> & 'static str { let target = target () ; if target . contains ("apple") { "dylib" } else if target . contains ("windows") { "dll" } else if target . contains ("aix") { "a" } else { "so" } }
};
}
