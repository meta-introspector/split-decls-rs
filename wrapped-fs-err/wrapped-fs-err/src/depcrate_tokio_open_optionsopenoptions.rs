// Generated macro for OpenOptions (struct)
macro_rules! Depcrate_tokio_open_optionsOpenOptions {
() => {
// Module: crate::tokio::open_options
// Provides: {"OpenOptions"}
// Dependencies: {}
# [doc = " Options and flags which can be used to configure how a file is opened."] # [doc = ""] # [doc = " This is a wrapper around [`tokio::fs::OpenOptions`]."] # [derive (Clone , Debug , Default)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub struct OpenOptions { tokio : TokioOpenOptions , }
};
}
