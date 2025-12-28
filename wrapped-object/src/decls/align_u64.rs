macro_rules! align_u64 {
    () => {
        # [allow (dead_code)] pub (crate) fn align_u64 (offset : u64 , size : u64) -> u64 { (offset + (size - 1)) & ! (size - 1) }
    };
}

align_u64!();