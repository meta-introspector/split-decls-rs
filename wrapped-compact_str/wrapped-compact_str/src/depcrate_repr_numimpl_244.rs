// Generated macro for impl_244 (impl)
macro_rules! Depcrate_repr_numimpl_244 {
() => {
// Module: crate::repr::num
// Provides: {"impl_244"}
// Dependencies: {}
impl NumChars for i8 { # [inline (always)] fn num_chars (val : i8) -> usize { match val { i8 :: MIN ..= - 100 => 4 , - 99 ..= - 10 => 3 , - 9 ..= - 1 => 2 , 0 ..= 9 => 1 , 10 ..= 99 => 2 , 100 ..= i8 :: MAX => 3 , } } }
};
}
