// Generated macro for dec_surrogates (function)
macro_rules! Depcratedec_surrogates {
() => {
// Module: crate
// Provides: {"dec_surrogates"}
// Dependencies: {}
# [doc = " Convert the bytes from a CESU-8 surrogate pair into a valid UTF-8"] # [doc = " sequence.  Assumes input is valid."] fn dec_surrogates (second : u8 , third : u8 , fifth : u8 , sixth : u8) -> [u8 ; 4] { let s1 = dec_surrogate (second , third) ; let s2 = dec_surrogate (fifth , sixth) ; let c = 0x10000 + (((s1 - 0xD800) << 10) | (s2 - 0xDC00)) ; assert ! (0x010000 <= c && c <= 0x10FFFF) ; [0b1111_0000u8 | ((c & 0b1_1100_0000_0000_0000_0000) >> 18) as u8 , TAG_CONT_U8 | ((c & 0b0_0011_1111_0000_0000_0000) >> 12) as u8 , TAG_CONT_U8 | ((c & 0b0_0000_0000_1111_1100_0000) >> 6) as u8 , TAG_CONT_U8 | ((c & 0b0_0000_0000_0000_0011_1111)) as u8] }
};
}
