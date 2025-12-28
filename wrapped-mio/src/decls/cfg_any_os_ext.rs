macro_rules! cfg_any_os_ext {
    () => {
        # [doc = " The `os-ext` feature is enabled, or one of the features that need `os-ext`."] macro_rules ! cfg_any_os_ext { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "os-ext" , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "os-ext" , feature = "net"))))] $ item) * } }
    };
}

cfg_any_os_ext!();