macro_rules! FALLBACK_LOOP_SIZE {
    () => {
        # [cfg (any (test , miri , not (target_arch = "x86_64")))] const FALLBACK_LOOP_SIZE : usize = 2 * USIZE_BYTES ;
    };
}

FALLBACK_LOOP_SIZE!()