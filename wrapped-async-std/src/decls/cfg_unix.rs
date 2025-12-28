macro_rules! cfg_unix {
    () => {
        # [doc = " Declares Unix-specific items."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_unix { ($ ($ item : item) *) => { $ (# [cfg (any (unix , feature = "docs"))] $ item) * } }
    };
}

cfg_unix!()