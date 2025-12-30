// Generated macro for static_lib_name (function)
macro_rules! Depcrate_artifact_namesstatic_lib_name {
() => {
// Module: crate::artifact_names
// Provides: {"static_lib_name"}
// Dependencies: {}
# [doc = " Construct the static library name based on the target."] # [track_caller] # [must_use] pub fn static_lib_name (name : & str) -> String { assert ! (! name . contains (char :: is_whitespace) , "static library name cannot contain whitespace") ; if is_windows_msvc () { format ! ("{name}.lib") } else { format ! ("lib{name}.a") } }
};
}
