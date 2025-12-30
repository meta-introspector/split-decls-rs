// Generated macro for impl_247 (impl)
macro_rules! Depcrate_repr_numimpl_247 {
() => {
// Module: crate::repr::num
// Provides: {"impl_247"}
// Dependencies: {}
impl NumChars for u32 { # [inline (always)] fn num_chars (val : u32) -> usize { match val { u32 :: MIN ..= 9 => 1 , 10 ..= 99 => 2 , 100 ..= 999 => 3 , 1000 ..= 9999 => 4 , 10000 ..= 99999 => 5 , 100000 ..= 999999 => 6 , 1000000 ..= 9999999 => 7 , 10000000 ..= 99999999 => 8 , 100000000 ..= 999999999 => 9 , 1000000000 ..= u32 :: MAX => 10 , } } }
};
}
