macro_rules! windows {
    () => {
        # [doc = " Windows implementation of dynamic library loading."] # [cfg (any (windows , libloading_docs))] # [cfg_attr (libloading_docs , doc (cfg (windows)))] pub mod windows ;
    };
}

windows!();