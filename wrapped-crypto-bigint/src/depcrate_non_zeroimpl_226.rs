// Generated macro for impl_226 (impl)
macro_rules! Depcrate_non_zeroimpl_226 {
() => {
// Module: crate::non_zero
// Provides: {"impl_226"}
// Dependencies: {}
impl < T > ConstOne for NonZero < T > where T : ConstOne + One , { const ONE : Self = Self (T :: ONE) ; }
};
}
