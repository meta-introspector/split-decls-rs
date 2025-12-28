macro_rules! windows {
    () => {
        # [cfg (all (windows , feature = "os-ext"))] # [cfg_attr (docsrs , doc (cfg (all (windows , feature = "os-ext"))))] pub mod windows { # ! [doc = " Windows only extensions."] pub use crate :: sys :: named_pipe :: NamedPipe ; }
    };
}

windows!();