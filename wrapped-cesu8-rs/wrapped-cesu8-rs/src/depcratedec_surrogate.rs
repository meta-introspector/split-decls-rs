// Generated macro for dec_surrogate (function)
macro_rules! Depcratedec_surrogate {
() => {
// Module: crate
// Provides: {"dec_surrogate"}
// Dependencies: {}
# [doc = " Convert the two trailing bytes from a CESU-8 surrogate to a regular"] # [doc = " surrogate value."] fn dec_surrogate (second : u8 , third : u8) -> u32 { 0xD000u32 | ((second & CONT_MASK) as u32) << 6 | (third & CONT_MASK) as u32 }
};
}
