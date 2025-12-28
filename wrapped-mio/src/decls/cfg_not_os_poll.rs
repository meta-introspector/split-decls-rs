macro_rules! cfg_not_os_poll {
    () => {
        # [doc = " The `os-poll` feature is disabled."] macro_rules ! cfg_not_os_poll { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "os-poll"))] $ item) * } }
    };
}

cfg_not_os_poll!();