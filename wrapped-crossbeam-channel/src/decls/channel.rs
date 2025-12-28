macro_rules! channel {
    () => {
        # [cfg (feature = "std")] mod channel ;
    };
}

channel!();