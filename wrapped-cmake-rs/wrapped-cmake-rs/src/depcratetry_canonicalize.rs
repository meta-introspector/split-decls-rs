// Generated macro for try_canonicalize (function)
macro_rules! Depcratetry_canonicalize {
() => {
// Module: crate
// Provides: {"try_canonicalize"}
// Dependencies: {}
# [doc = " Attempt to canonicalize; fall back to the original path if unsuccessful, in case `cmake` knows"] # [doc = " something we don't."] fn try_canonicalize (path : & Path) -> PathBuf { path . canonicalize () . unwrap_or_else (| _ | path . to_owned ()) }
};
}
