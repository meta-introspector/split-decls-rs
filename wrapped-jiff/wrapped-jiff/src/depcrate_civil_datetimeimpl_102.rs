// Generated macro for impl_102 (impl)
macro_rules! Depcrate_civil_datetimeimpl_102 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_102"}
// Dependencies: {}
# [doc = " Computes the span of time between two datetimes."] # [doc = ""] # [doc = " This will return a negative span when the datetime being subtracted is"] # [doc = " greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two datetimes (no rounding and largest units is days), this will never"] # [doc = " panic or fail in any way. It is guaranteed that the largest non-zero"] # [doc = " unit in the `Span` returned will be days."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`DateTime::since`]."] # [doc = ""] # [doc = " If you need a [`SignedDuration`] representing the span between two civil"] # [doc = " datetimes, then use [`DateTime::duration_since`]."] impl core :: ops :: Sub for DateTime { type Output = Span ; # [inline] fn sub (self , rhs : DateTime) -> Span { self . since (rhs) . expect ("since never fails when given DateTime") } }
};
}
