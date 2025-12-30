// Generated macro for is_oct_digit (function)
macro_rules! Depcrate_characteris_oct_digit {
() => {
// Module: crate::character
// Provides: {"is_oct_digit"}
// Dependencies: {}
# [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_oct_digit`")] pub fn is_oct_digit (chr : u8) -> bool { matches ! (chr , 0x30 ..= 0x37) }
};
}
