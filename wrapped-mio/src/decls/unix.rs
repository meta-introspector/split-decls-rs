macro_rules! unix {
    () => {
        # [cfg (all (unix , feature = "os-ext"))] # [cfg_attr (docsrs , doc (cfg (all (unix , feature = "os-ext"))))] pub mod unix { # ! [doc = " Unix only extensions."] pub mod pipe { # ! [doc = " Unix pipe."] # ! [doc = ""] # ! [doc = " See the [`new`] function for documentation."] pub use crate :: sys :: pipe :: { new , Receiver , Sender } ; } pub use crate :: sys :: SourceFd ; }
    };
}

unix!()