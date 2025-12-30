// Generated macro for byte2hex (function)
macro_rules! Depcratebyte2hex {
() => {
// Module: crate
// Provides: {"byte2hex"}
// Dependencies: {}
# [inline] # [must_use] fn byte2hex (byte : u8 , table : & [u8 ; 16]) -> (u8 , u8) { let high = table [((byte & 0xf0) >> 4) as usize] ; let low = table [(byte & 0x0f) as usize] ; (high , low) }
};
}
