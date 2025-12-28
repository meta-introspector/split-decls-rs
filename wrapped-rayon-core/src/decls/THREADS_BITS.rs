macro_rules! THREADS_BITS {
    () => {
        # [cfg (target_pointer_width = "32")] const THREADS_BITS : usize = 8 ;
    };
}

THREADS_BITS!();