// Generated macro for impl_760 (impl)
macro_rules! Depcrate_spanimpl_760 {
() => {
// Module: crate::span
// Provides: {"impl_760"}
// Dependencies: {}
# [doc = " This multiplies each unit in a span by an integer."] # [doc = ""] # [doc = " This panics on overflow. For checked arithmetic, use [`Span::checked_mul`]."] impl core :: ops :: Mul < Span > for i64 { type Output = Span ; # [inline] fn mul (self , rhs : Span) -> Span { rhs . checked_mul (self) . expect ("multiplying `Span` by a scalar overflowed") } }
};
}
