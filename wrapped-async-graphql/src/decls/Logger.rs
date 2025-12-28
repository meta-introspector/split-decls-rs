macro_rules! Logger {
    () => {
        # [doc = " Logger extension"] # [cfg_attr (docsrs , doc (cfg (feature = "log")))] pub struct Logger ;
    };
}

Logger!();