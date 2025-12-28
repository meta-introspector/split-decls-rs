macro_rules! LATIN1_MASK {
    () => {
        # [allow (dead_code)] const LATIN1_MASK : usize = 0xFF00_FF00_FF00_FF00u64 as usize ;
    };
}

LATIN1_MASK!();