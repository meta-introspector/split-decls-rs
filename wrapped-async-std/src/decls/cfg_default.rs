macro_rules! cfg_default {
    () => {
        # [doc = " Declares default items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_default { ($ ($ item : item) *) => { $ (# [cfg (feature = "default")] $ item) * } }
    };
}

cfg_default!();