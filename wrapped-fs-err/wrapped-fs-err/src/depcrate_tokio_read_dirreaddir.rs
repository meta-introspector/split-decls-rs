// Generated macro for ReadDir (struct)
macro_rules! Depcrate_tokio_read_dirReadDir {
() => {
// Module: crate::tokio::read_dir
// Provides: {"ReadDir"}
// Dependencies: {}
# [doc = " Reads the entries in a directory."] # [doc = ""] # [doc = " This is a wrapper around [`tokio::fs::ReadDir`]."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub struct ReadDir { tokio : fs :: ReadDir , path : PathBuf , }
};
}
