// Generated macro for impl_759 (impl)
macro_rules! Depcrate_spanimpl_759 {
() => {
// Module: crate::span
// Provides: {"impl_759"}
// Dependencies: {}
# [doc = " This multiplies each unit in a span by an integer."] # [doc = ""] # [doc = " This panics on overflow. For checked arithmetic, use [`Span::checked_mul`]."] impl core :: ops :: Mul < i64 > for Span { type Output = Span ; # [inline] fn mul (self , rhs : i64) -> Span { self . checked_mul (rhs) . expect ("multiplying `Span` by a scalar overflowed") } }
};
}
