// Generated macro for DirEntry (struct)
macro_rules! Depcrate_tokio_read_dirDirEntry {
() => {
// Module: crate::tokio::read_dir
// Provides: {"DirEntry"}
// Dependencies: {}
# [doc = " Entries returned by the [`ReadDir`] stream."] # [doc = ""] # [doc = " This is a wrapper around [`tokio::fs::DirEntry`]."] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub struct DirEntry { tokio : fs :: DirEntry , }
};
}
