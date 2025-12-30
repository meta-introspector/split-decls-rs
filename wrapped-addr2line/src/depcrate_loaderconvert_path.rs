// Generated macro for convert_path (function)
macro_rules! Depcrate_loaderconvert_path {
() => {
// Module: crate::loader
// Provides: {"convert_path"}
// Dependencies: {}
# [cfg (not (unix))] fn convert_path (bytes : & [u8]) -> Result < PathBuf > { let s = std :: str :: from_utf8 (bytes) ? ; Ok (PathBuf :: from (s)) }
};
}
