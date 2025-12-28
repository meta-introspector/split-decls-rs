macro_rules! big5_astral_encode {
    () => {
        # [inline (always)] pub fn big5_astral_encode (low_bits : u16) -> Option < usize > { match low_bits { 0x00CC => Some (11205 - 942) , 0x008A => Some (11207 - 942) , 0x7607 => Some (11213 - 942) , _ => { let mut i = 18997 - 942 ; while i < BIG5_LOW_BITS . len () - 1 { if BIG5_LOW_BITS [i] == low_bits && big5_is_astral (i) { return Some (i) ; } i += 1 ; } None } } }
    };
}

big5_astral_encode!()