macro_rules! linux {
    () => {
        # [cfg (any (linux_android , target_os = "fuchsia" , target_os = "redox"))] # [macro_use] mod linux ;
    };
}

linux!();