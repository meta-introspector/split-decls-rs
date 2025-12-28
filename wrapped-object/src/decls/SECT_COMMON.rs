macro_rules! SECT_COMMON {
    () => {
        # [doc = " the section common symbols are allocated in by the link editor"] pub const SECT_COMMON : & str = "__common" ;
    };
}

SECT_COMMON!();