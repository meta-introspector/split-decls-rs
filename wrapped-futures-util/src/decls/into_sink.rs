macro_rules! into_sink {
    () => {
        # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] mod into_sink ;
    };
}

into_sink!()