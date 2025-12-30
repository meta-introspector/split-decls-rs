// Generated macro for is_pad_ct (function)
macro_rules! Depcrate_encodingis_pad_ct {
() => {
// Module: crate::encoding
// Provides: {"is_pad_ct"}
// Dependencies: {}
# [doc = " Branchless match that a given byte is the `PAD` character"] # [allow (clippy :: arithmetic_side_effects)] # [inline (always)] fn is_pad_ct (input : u8) -> i16 { ((((PAD as i16 - 1) - input as i16) & (input as i16 - (PAD as i16 + 1))) >> 8) & 1 }
};
}
