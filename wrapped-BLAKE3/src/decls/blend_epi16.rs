macro_rules! blend_epi16 {
    () => {
        # [inline (always)] fn blend_epi16 (a : v128 , b : v128 , imm8 : i32) -> v128 { let bits = i16x8 (0x01 , 0x02 , 0x04 , 0x08 , 0x10 , 0x20 , 0x40 , 0x80) ; let mut mask = i16x8_splat (imm8 as i16) ; mask = v128_and (mask , bits) ; mask = i16x8_eq (mask , bits) ; v128_bitselect (b , a , mask) }
    };
}

blend_epi16!();