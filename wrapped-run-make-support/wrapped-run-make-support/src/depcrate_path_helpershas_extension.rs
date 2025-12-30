// Generated macro for has_extension (function)
macro_rules! Depcrate_path_helpershas_extension {
() => {
// Module: crate::path_helpers
// Provides: {"has_extension"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` has the extension `extension`."] pub fn has_extension < P : AsRef < Path > > (path : P , extension : & str) -> bool { path . as_ref () . extension () . is_some_and (| ext | ext == extension) }
};
}
