// Generated macro for impl_114 (impl)
macro_rules! Depcrate_devimpl_114 {
() => {
// Module: crate::dev
// Provides: {"impl_114"}
// Dependencies: {}
impl ConditionallySelectable for ProjectivePoint { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { if choice . into () { * b } else { * a } } }
};
}
