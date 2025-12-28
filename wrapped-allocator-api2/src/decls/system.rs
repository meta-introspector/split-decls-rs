macro_rules! system {
    () => {
        # [cfg (feature = "std")] mod system ;
    };
}

system!()