macro_rules! unix {
    () => {
        # [doc = " UNIX implementation of dynamic library loading."] # [cfg (any (unix , libloading_docs))] # [cfg_attr (libloading_docs , doc (cfg (unix)))] pub mod unix ;
    };
}

unix!()