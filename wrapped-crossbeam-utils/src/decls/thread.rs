macro_rules! thread {
    () => {
        # [cfg (feature = "std")] # [cfg (not (crossbeam_loom))] pub mod thread ;
    };
}

thread!()