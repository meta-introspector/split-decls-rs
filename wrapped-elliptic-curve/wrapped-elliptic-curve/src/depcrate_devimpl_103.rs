// Generated macro for impl_103 (impl)
macro_rules! Depcrate_devimpl_103 {
() => {
// Module: crate::dev
// Provides: {"impl_103"}
// Dependencies: {}
impl ConditionallySelectable for AffinePoint { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { if choice . into () { * b } else { * a } } }
};
}
