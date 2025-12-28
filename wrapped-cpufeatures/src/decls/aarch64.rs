macro_rules! aarch64 {
    () => {
        # [cfg (not (miri))] # [cfg (target_arch = "aarch64")] # [doc (hidden)] pub mod aarch64 ;
    };
}

aarch64!()