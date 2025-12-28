macro_rules! basic {
    () => {
        # [cfg (feature = "basic-scheme")] # [cfg_attr (docsrs , doc (cfg (feature = "basic-scheme")))] pub mod basic ;
    };
}

basic!()