macro_rules! cfg_docs {
    () => {
        # [doc = " Declares items when the \"docs\" feature is enabled."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_docs { ($ ($ item : item) *) => { $ (# [cfg (feature = "docs")] $ item) * } }
    };
}

cfg_docs!()