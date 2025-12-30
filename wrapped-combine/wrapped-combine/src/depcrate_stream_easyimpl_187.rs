// Generated macro for impl_187 (impl)
macro_rules! Depcrate_stream_easyimpl_187 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_187"}
// Dependencies: {}
impl < S > Positioned for Stream < S > where S : StreamOnce + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , { fn position (& self) -> S :: Position { self . 0 . position () } }
};
}
