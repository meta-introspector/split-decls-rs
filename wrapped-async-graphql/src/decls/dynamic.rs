macro_rules! dynamic {
    () => {
        # [cfg (feature = "dynamic-schema")] # [cfg_attr (docsrs , doc (cfg (feature = "dynamic-schema")))] pub mod dynamic ;
    };
}

dynamic!();