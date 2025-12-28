macro_rules! digest {
    () => {
        # [cfg (feature = "digest-scheme")] # [cfg_attr (docsrs , doc (cfg (feature = "digest-scheme")))] pub mod digest ;
    };
}

digest!();