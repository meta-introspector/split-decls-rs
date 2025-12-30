// Generated macro for strip_prefix (function)
macro_rules! Depcrate_pathutilstrip_prefix {
() => {
// Module: crate::pathutil
// Provides: {"strip_prefix"}
// Dependencies: {}
# [doc = " Strip `prefix` from the `path` and return the remainder."] # [doc = ""] # [doc = " If `path` doesn't have a prefix `prefix`, then return `None`."] # [cfg (not (unix))] pub (crate) fn strip_prefix < 'a , P : AsRef < Path > + ? Sized > (prefix : & 'a P , path : & 'a Path ,) -> Option < & 'a Path > { path . strip_prefix (prefix) . ok () }
};
}
