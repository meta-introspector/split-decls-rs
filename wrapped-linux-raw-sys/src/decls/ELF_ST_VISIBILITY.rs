macro_rules! ELF_ST_VISIBILITY {
    () => {
        # [inline] pub const fn ELF_ST_VISIBILITY (o : u8) -> u8 { o & 0x03 }
    };
}

ELF_ST_VISIBILITY!();