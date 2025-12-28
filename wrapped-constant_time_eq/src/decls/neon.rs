macro_rules! neon {
    () => {
        # [cfg (all (target_arch = "aarch64" , target_feature = "neon" , not (miri)))] mod neon ;
    };
}

neon!()