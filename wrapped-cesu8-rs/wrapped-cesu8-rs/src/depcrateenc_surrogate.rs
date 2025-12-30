// Generated macro for enc_surrogate (function)
macro_rules! Depcrateenc_surrogate {
() => {
// Module: crate
// Provides: {"enc_surrogate"}
// Dependencies: {}
# [doc = " Encode a single surrogate as CESU-8."] fn enc_surrogate (surrogate : u16) -> [u8 ; 3] { assert ! (0xD800 <= surrogate && surrogate <= 0xDFFF) ; [0b11100000 | ((surrogate & 0b11110000_00000000) >> 12) as u8 , TAG_CONT_U8 | ((surrogate & 0b00001111_11000000) >> 6) as u8 , TAG_CONT_U8 | ((surrogate & 0b00000000_00111111)) as u8] }
};
}
