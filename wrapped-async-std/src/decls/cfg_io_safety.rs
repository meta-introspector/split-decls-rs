macro_rules! cfg_io_safety {
    () => {
        # [doc = " Declares items that use I/O safety."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_io_safety { ($ ($ item : item) *) => { $ (# [cfg (feature = "io_safety")] $ item) * } }
    };
}

cfg_io_safety!()