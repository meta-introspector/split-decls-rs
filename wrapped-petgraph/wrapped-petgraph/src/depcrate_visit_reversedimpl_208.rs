// Generated macro for impl_208 (impl)
macro_rules! Depcrate_visit_reversedimpl_208 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_208"}
// Dependencies: {}
impl < R > ReversedEdgeReference < R > { # [doc = " Return the original, unreversed edge reference."] pub fn as_unreversed (& self) -> & R { & self . 0 } # [doc = " Consume `self` and return the original, unreversed edge reference."] pub fn into_unreversed (self) -> R { self . 0 } }
};
}
