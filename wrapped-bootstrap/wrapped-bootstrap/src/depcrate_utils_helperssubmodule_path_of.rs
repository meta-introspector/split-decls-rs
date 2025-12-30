// Generated macro for submodule_path_of (function)
macro_rules! Depcrate_utils_helperssubmodule_path_of {
() => {
// Module: crate::utils::helpers
// Provides: {"submodule_path_of"}
// Dependencies: {}
# [doc = " Return the path to the containing submodule if available."] pub fn submodule_path_of (builder : & Builder < '_ > , path : & str) -> Option < String > { let submodule_paths = builder . submodule_paths () ; submodule_paths . iter () . find_map (| submodule_path | { if path . starts_with (submodule_path) { Some (submodule_path . to_string ()) } else { None } }) }
};
}
