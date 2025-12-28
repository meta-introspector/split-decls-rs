macro_rules! bilock {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (any (feature = "bilock" , feature = "sink" , feature = "io"))] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] # [cfg_attr (not (feature = "bilock") , allow (unreachable_pub))] mod bilock ;
    };
}

bilock!()