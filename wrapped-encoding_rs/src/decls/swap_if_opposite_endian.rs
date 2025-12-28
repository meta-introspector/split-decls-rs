macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! swap_if_opposite_endian {
    () => {
        deps!();
        # [inline (always)] fn swap_if_opposite_endian < E : Endian > (unit : u16) -> u16 { if E :: OPPOSITE_ENDIAN { unit . swap_bytes () } else { unit } }
    };
}

swap_if_opposite_endian!();