// Generated macro for impl_245 (impl)
macro_rules! Depcrate_repr_numimpl_245 {
() => {
// Module: crate::repr::num
// Provides: {"impl_245"}
// Dependencies: {}
impl NumChars for u16 { # [inline (always)] fn num_chars (val : u16) -> usize { match val { u16 :: MIN ..= 9 => 1 , 10 ..= 99 => 2 , 100 ..= 999 => 3 , 1000 ..= 9999 => 4 , 10000 ..= u16 :: MAX => 5 , } } }
};
}
