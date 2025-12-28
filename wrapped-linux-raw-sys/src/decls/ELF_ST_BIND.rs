macro_rules! ELF_ST_BIND {
    () => {
        # [inline] pub const fn ELF_ST_BIND (val : u8) -> u8 { val >> 4 }
    };
}

ELF_ST_BIND!()