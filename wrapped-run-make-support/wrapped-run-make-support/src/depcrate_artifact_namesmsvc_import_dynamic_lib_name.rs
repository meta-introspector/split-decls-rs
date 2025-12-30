// Generated macro for msvc_import_dynamic_lib_name (function)
macro_rules! Depcrate_artifact_namesmsvc_import_dynamic_lib_name {
() => {
// Module: crate::artifact_names
// Provides: {"msvc_import_dynamic_lib_name"}
// Dependencies: {}
# [doc = " Construct the name of the import library for the dynamic library, exclusive to MSVC and accepted"] # [doc = " by link.exe."] # [track_caller] # [must_use] pub fn msvc_import_dynamic_lib_name (name : & str) -> String { assert ! (is_windows_msvc () , "this function is exclusive to MSVC") ; assert ! (! name . contains (char :: is_whitespace) , "import library name cannot contain whitespace") ; format ! ("{name}.dll.lib") }
};
}
