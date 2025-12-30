// Generated macro for dynamic_lib_name (function)
macro_rules! Depcrate_artifact_namesdynamic_lib_name {
() => {
// Module: crate::artifact_names
// Provides: {"dynamic_lib_name"}
// Dependencies: {}
# [doc = " Construct the dynamic library name based on the target."] # [track_caller] # [must_use] pub fn dynamic_lib_name (name : & str) -> String { assert ! (! name . contains (char :: is_whitespace) , "dynamic library name cannot contain whitespace") ; format ! ("{}{name}.{}" , dynamic_lib_prefix () , dynamic_lib_extension ()) }
};
}
