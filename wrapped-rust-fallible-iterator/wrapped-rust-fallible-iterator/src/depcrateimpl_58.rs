// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < I : DoubleEndedFallibleIterator + ? Sized > DoubleEndedFallibleIterator for & mut I { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next_back () } }
};
}
