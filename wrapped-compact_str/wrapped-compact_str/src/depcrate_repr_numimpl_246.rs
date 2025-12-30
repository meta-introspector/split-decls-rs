// Generated macro for impl_246 (impl)
macro_rules! Depcrate_repr_numimpl_246 {
() => {
// Module: crate::repr::num
// Provides: {"impl_246"}
// Dependencies: {}
impl NumChars for i16 { # [inline (always)] fn num_chars (val : i16) -> usize { match val { i16 :: MIN ..= - 10000 => 6 , - 9999 ..= - 1000 => 5 , - 999 ..= - 100 => 4 , - 99 ..= - 10 => 3 , - 9 ..= - 1 => 2 , 0 ..= 9 => 1 , 10 ..= 99 => 2 , 100 ..= 999 => 3 , 1000 ..= 9999 => 4 , 10000 ..= i16 :: MAX => 5 , } } }
};
}
