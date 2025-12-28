macro_rules! macro_99 {
    () => {
        # [cfg (target_pointer_width = "64")] wmul_impl_usize ! { u64 }
    };
}

macro_99!();