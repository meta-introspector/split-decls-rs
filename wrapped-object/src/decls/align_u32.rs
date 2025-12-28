macro_rules! align_u32 {
    () => {
        # [allow (dead_code)] pub (crate) fn align_u32 (offset : u32 , size : u32) -> u32 { (offset + (size - 1)) & ! (size - 1) }
    };
}

align_u32!()