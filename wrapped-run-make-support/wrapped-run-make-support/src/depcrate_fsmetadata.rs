// Generated macro for metadata (function)
macro_rules! Depcrate_fsmetadata {
() => {
// Module: crate::fs
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::metadata`] which includes the file path in the panic message. Note"] # [doc = " that this will traverse symlinks and will return metadata about the target file. Use"] # [doc = " [`symlink_metadata`] if you don't want to traverse symlinks."] # [doc = ""] # [doc = " See [`std::fs::metadata`] docs for more details."] # [track_caller] pub fn metadata < P : AsRef < Path > > (path : P) -> std :: fs :: Metadata { match std :: fs :: metadata (path . as_ref ()) { Ok (m) => m , Err (e) => panic ! ("failed to read file metadata at `{}`: {e}" , path . as_ref () . display ()) , } }
};
}
