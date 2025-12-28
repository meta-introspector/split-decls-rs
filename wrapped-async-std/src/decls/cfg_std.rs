macro_rules! cfg_std {
    () => {
        # [doc = " Declares std items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_std { ($ ($ item : item) *) => { $ (# [cfg (feature = "std")] $ item) * } }
    };
}

cfg_std!();