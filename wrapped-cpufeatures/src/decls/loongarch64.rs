macro_rules! loongarch64 {
    () => {
        # [cfg (not (miri))] # [cfg (target_arch = "loongarch64")] # [doc (hidden)] pub mod loongarch64 ;
    };
}

loongarch64!();