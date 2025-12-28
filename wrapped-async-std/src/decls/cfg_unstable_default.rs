macro_rules! cfg_unstable_default {
    () => {
        # [doc = " Declares unstable and default items."] # [doc (hidden)] macro_rules ! cfg_unstable_default { ($ ($ item : item) *) => { $ (# [cfg (all (feature = "default" , feature = "unstable"))] # [cfg_attr (feature = "docs" , doc (cfg (unstable)))] $ item) * } }
    };
}

cfg_unstable_default!();