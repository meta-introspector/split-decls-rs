macro_rules! compat {
    () => {
        # [cfg (feature = "compat")] # [cfg_attr (docsrs , doc (cfg (feature = "compat")))] pub mod compat ;
    };
}

compat!()