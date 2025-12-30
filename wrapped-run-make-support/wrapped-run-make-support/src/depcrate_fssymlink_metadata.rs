// Generated macro for symlink_metadata (function)
macro_rules! Depcrate_fssymlink_metadata {
() => {
// Module: crate::fs
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::symlink_metadata`] which includes the file path in the panic"] # [doc = " message. Note that this will not traverse symlinks and will return metadata about the filesystem"] # [doc = " entity itself. Use [`metadata`] if you want to traverse symlinks."] # [doc = ""] # [doc = " See [`std::fs::symlink_metadata`] docs for more details."] # [track_caller] pub fn symlink_metadata < P : AsRef < Path > > (path : P) -> std :: fs :: Metadata { match std :: fs :: symlink_metadata (path . as_ref ()) { Ok (m) => m , Err (e) => { panic ! ("failed to read file metadata (shallow) at `{}`: {e}" , path . as_ref () . display ()) } } }
};
}
