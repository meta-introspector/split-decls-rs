macro_rules! x86 {
    () => {
        # [cfg (not (miri))] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] mod x86 ;
    };
}

x86!();