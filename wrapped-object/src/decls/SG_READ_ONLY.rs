macro_rules! SG_READ_ONLY {
    () => {
        # [doc = " This segment is made read-only after fixups"] pub const SG_READ_ONLY : u32 = 0x10 ;
    };
}

SG_READ_ONLY!()