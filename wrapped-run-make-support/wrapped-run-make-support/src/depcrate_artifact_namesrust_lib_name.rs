// Generated macro for rust_lib_name (function)
macro_rules! Depcrate_artifact_namesrust_lib_name {
() => {
// Module: crate::artifact_names
// Provides: {"rust_lib_name"}
// Dependencies: {}
# [doc = " Construct the name of a rust library (rlib)."] # [track_caller] # [must_use] pub fn rust_lib_name (name : & str) -> String { format ! ("lib{name}.rlib") }
};
}
