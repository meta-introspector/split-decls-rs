// Generated macro for impl_1394 (impl)
macro_rules! Depcrate_zonedimpl_1394 {
() => {
// Module: crate::zoned
// Provides: {"impl_1394"}
// Dependencies: {}
# [doc = " Computes the span of time between two borrowed zoned datetimes."] # [doc = ""] # [doc = " This will return a negative span when the zoned datetime being subtracted"] # [doc = " is greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two zoned datetimes (no rounding and largest units is hours), this will"] # [doc = " never panic or fail in any way. It is guaranteed that the largest non-zero"] # [doc = " unit in the `Span` returned will be hours."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`Zoned::since`]."] impl < 'a > core :: ops :: Sub for & 'a Zoned { type Output = Span ; # [inline] fn sub (self , rhs : & 'a Zoned) -> Span { self . since (rhs) . expect ("since never fails when given Zoned") } }
};
}
