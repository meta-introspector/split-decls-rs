macro_rules! sink {
    () => {
        # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub mod sink ;
    };
}

sink!()