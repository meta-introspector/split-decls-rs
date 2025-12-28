macro_rules! cfg_windows {
    () => {
        # [doc = " Declares Windows-specific items."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_windows { ($ ($ item : item) *) => { $ (# [cfg (any (windows , feature = "docs"))] $ item) * } }
    };
}

cfg_windows!();