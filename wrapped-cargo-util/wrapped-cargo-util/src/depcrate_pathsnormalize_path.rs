// Generated macro for normalize_path (function)
macro_rules! Depcrate_pathsnormalize_path {
() => {
// Module: crate::paths
// Provides: {"normalize_path"}
// Dependencies: {}
# [doc = " Normalize a path, removing things like `.` and `..`."] # [doc = ""] # [doc = " CAUTION: This does not resolve symlinks (unlike"] # [doc = " [`std::fs::canonicalize`]). This may cause incorrect or surprising"] # [doc = " behavior at times. This should be used carefully. Unfortunately,"] # [doc = " [`std::fs::canonicalize`] can be hard to use correctly, since it can often"] # [doc = " fail, or on Windows returns annoying device paths. This is a problem Cargo"] # [doc = " needs to improve on."] pub fn normalize_path (path : & Path) -> PathBuf { let mut components = path . components () . peekable () ; let mut ret = if let Some (c @ Component :: Prefix (..)) = components . peek () . cloned () { components . next () ; PathBuf :: from (c . as_os_str ()) } else { PathBuf :: new () } ; for component in components { match component { Component :: Prefix (..) => unreachable ! () , Component :: RootDir => { ret . push (Component :: RootDir) ; } Component :: CurDir => { } Component :: ParentDir => { if ret . ends_with (Component :: ParentDir) { ret . push (Component :: ParentDir) ; } else { let popped = ret . pop () ; if ! popped && ! ret . has_root () { ret . push (Component :: ParentDir) ; } } } Component :: Normal (c) => { ret . push (c) ; } } } ret }
};
}
