macro_rules! cfg_io_source {
    () => {
        # [doc = " One of the features enabled that needs `IoSource`. That is `net` or `os-ext`"] # [doc = " on Unix (for `pipe`)."] macro_rules ! cfg_io_source { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , all (unix , feature = "os-ext")))] # [cfg_attr (docsrs , doc (cfg (any (feature = "net" , all (unix , feature = "os-ext")))))] $ item) * } }
    };
}

cfg_io_source!();