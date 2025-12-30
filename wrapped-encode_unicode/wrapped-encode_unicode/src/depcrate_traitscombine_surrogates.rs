// Generated macro for combine_surrogates (function)
macro_rules! Depcrate_traitscombine_surrogates {
() => {
// Module: crate::traits
// Provides: {"combine_surrogates"}
// Dependencies: {}
# [doc = " Create a `char` from a leading and a trailing surrogate."] # [doc = ""] # [doc = " This function is safe because it ignores the six most significant bits of"] # [doc = " each argument and always produces a codepoint in `0x01_00_00..=0x10_ff_ff`."] fn combine_surrogates (first : u16 , second : u16) -> char { unsafe { let high = (first & 0x_03_ff) as u32 ; let low = (second & 0x_03_ff) as u32 ; let c = ((high << 10) | low) + 0x_01_00_00 ; char :: from_u32_unchecked (c) } }
};
}
