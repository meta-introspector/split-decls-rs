macro_rules! x86_64 {
    () => {
        # [cfg (target_arch = "x86_64")] pub mod x86_64 ;
    };
}

x86_64!()