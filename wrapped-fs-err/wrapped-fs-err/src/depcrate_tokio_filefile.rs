// Generated macro for File (struct)
macro_rules! Depcrate_tokio_fileFile {
() => {
// Module: crate::tokio::file
// Provides: {"File"}
// Dependencies: {}
# [doc = " Wrapper around [`tokio::fs::File`] which adds more helpful"] # [doc = " information to all errors."] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub struct File { tokio : fs :: File , path : PathBuf , }
};
}
