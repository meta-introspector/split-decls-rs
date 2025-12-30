// Generated macro for impl_60 (impl)
macro_rules! Depcrate_devimpl_60 {
() => {
// Module: crate::dev
// Provides: {"impl_60"}
// Dependencies: {}
impl ConditionallySelectable for Scalar { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (ScalarValue :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
