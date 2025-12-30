// Generated macro for char_from_u16 (function)
macro_rules! Depcratechar_from_u16 {
() => {
// Module: crate
// Provides: {"char_from_u16"}
// Dependencies: {}
# [doc = " Convert a `u16` _obtained from data provider data_ to `char`."] # [inline (always)] fn char_from_u16 (u : u16) -> char { char_from_u32 (u32 :: from (u)) }
};
}
