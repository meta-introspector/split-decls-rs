macro_rules! cfg_os_poll {
    () => {
        # [doc = " The `os-poll` feature is enabled."] macro_rules ! cfg_os_poll { ($ ($ item : item) *) => { $ (# [cfg (feature = "os-poll")] # [cfg_attr (docsrs , doc (cfg (feature = "os-poll")))] $ item) * } }
    };
}

cfg_os_poll!()