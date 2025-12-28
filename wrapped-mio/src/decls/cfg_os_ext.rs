macro_rules! cfg_os_ext {
    () => {
        # [doc = " The `os-ext` feature is enabled."] macro_rules ! cfg_os_ext { ($ ($ item : item) *) => { $ (# [cfg (feature = "os-ext")] # [cfg_attr (docsrs , doc (cfg (feature = "os-ext")))] $ item) * } }
    };
}

cfg_os_ext!();