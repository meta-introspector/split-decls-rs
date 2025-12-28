macro_rules! ELF_ST_TYPE {
    () => {
        # [inline] pub const fn ELF_ST_TYPE (val : u8) -> u8 { val & 0xf }
    };
}

ELF_ST_TYPE!()