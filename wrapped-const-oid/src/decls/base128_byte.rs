macro_rules! deps {
    () => {
        Result!();
        Arc!();
    };
}

macro_rules! base128_byte {
    () => {
        deps!();
        # [doc = " Compute the big endian base 128 encoding of the given [`Arc`] at the given byte."] const fn base128_byte (arc : Arc , pos : usize , total : usize) -> Result < u8 > { debug_assert ! (pos < total) ; let last_byte = checked_add ! (pos , 1) == total ; let mask = if last_byte { 0 } else { 0b10000000 } ; let shift = checked_mul ! (checked_sub ! (checked_sub ! (total , pos) , 1) , 7) ; Ok (((arc >> shift) & 0b1111111) as u8 | mask) }
    };
}

base128_byte!()