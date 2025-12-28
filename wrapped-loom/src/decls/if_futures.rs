macro_rules! if_futures {
    () => {
        macro_rules ! if_futures { ($ ($ t : tt) *) => { cfg_if :: cfg_if ! { if # [cfg (feature = "futures")] { # [cfg_attr (docsrs , doc (cfg (feature = "futures")))] $ ($ t) * } } } }
    };
}

if_futures!();