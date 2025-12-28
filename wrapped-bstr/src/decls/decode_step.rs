macro_rules! decode_step {
    () => {
        # [doc = " SAFETY: The decode function relies on state being equal to ACCEPT only if"] # [doc = " cp is a valid Unicode scalar value."] # [inline] pub fn decode_step (state : & mut usize , cp : & mut u32 , b : u8) { let class = CLASSES [b as usize] ; let b = u32 :: from (b) ; if * state == ACCEPT { * cp = (0xFF >> class) & b ; } else { * cp = (b & 0b0011_1111) | (* cp << 6) ; } * state = STATES_FORWARD [* state + class as usize] as usize ; }
    };
}

decode_step!();