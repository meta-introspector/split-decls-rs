macro_rules! aarch64 {
    () => {
        # [cfg (target_arch = "aarch64")] pub mod aarch64 ;
    };
}

aarch64!();