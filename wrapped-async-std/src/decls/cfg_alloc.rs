macro_rules! cfg_alloc {
    () => {
        # [doc = " Declares no-std items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_alloc { ($ ($ item : item) *) => { $ (# [cfg (feature = "alloc")] $ item) * } }
    };
}

cfg_alloc!()