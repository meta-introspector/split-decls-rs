macro_rules! Wide {
    () => {
        # [cfg (not (all (target_pointer_width = "64" , not (target_arch = "sparc"))))] pub type Wide = u64 ;
    };
}

Wide!();