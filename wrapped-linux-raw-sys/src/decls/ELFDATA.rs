macro_rules! ELFDATA {
    () => {
        # [cfg (target_endian = "big")] pub const ELFDATA : u8 = 2 ;
    };
}

ELFDATA!()