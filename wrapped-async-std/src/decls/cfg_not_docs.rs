macro_rules! cfg_not_docs {
    () => {
        # [doc = " Declares items when the \"docs\" feature is disabled."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_not_docs { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "docs"))] $ item) * } }
    };
}

cfg_not_docs!();