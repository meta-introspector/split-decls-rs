// Generated macro for is_dylib (function)
macro_rules! Depcrate_utils_helpersis_dylib {
() => {
// Module: crate::utils::helpers
// Provides: {"is_dylib"}
// Dependencies: {}
# [doc = " Returns `true` if the file name given looks like a dynamic library."] pub fn is_dylib (path : & Path) -> bool { path . extension () . and_then (| ext | ext . to_str ()) . is_some_and (| ext | { ext == "dylib" || ext == "so" || ext == "dll" || (ext == "a" && is_aix_shared_archive (path)) }) }
};
}
