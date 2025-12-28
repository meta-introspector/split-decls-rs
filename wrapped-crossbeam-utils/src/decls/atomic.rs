macro_rules! atomic {
    () => {
        # [cfg (feature = "atomic")] # [cfg_attr (docsrs , doc (cfg (feature = "atomic")))] pub mod atomic ;
    };
}

atomic!()