// Generated macro for impl_228 (impl)
macro_rules! Depcrate_non_zeroimpl_228 {
() => {
// Module: crate::non_zero
// Provides: {"impl_228"}
// Dependencies: {}
impl < T > num_traits :: One for NonZero < T > where T : One + Mul < T , Output = T > , { # [inline] fn one () -> Self { Self (T :: one ()) } fn is_one (& self) -> bool { self . 0 . is_one () . into () } }
};
}
