// Generated macro for impl_287 (impl)
macro_rules! Depcrate_oddimpl_287 {
() => {
// Module: crate::odd
// Provides: {"impl_287"}
// Dependencies: {}
impl < T > num_traits :: One for Odd < T > where T : One + Mul < T , Output = T > , { # [inline] fn one () -> Self { Self (T :: one ()) } fn is_one (& self) -> bool { self . 0 . is_one () . into () } }
};
}
