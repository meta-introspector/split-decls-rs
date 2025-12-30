// Generated macro for find_library (function)
macro_rules! Depcratefind_library {
() => {
// Module: crate
// Provides: {"find_library"}
// Dependencies: {}
# [doc = " Deprecated in favor of the probe_library function"] # [doc (hidden)] pub fn find_library (name : & str) -> Result < Library , String > { probe_library (name) . map_err (| e | e . to_string ()) }
};
}
