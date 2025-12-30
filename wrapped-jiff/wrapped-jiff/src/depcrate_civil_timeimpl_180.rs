// Generated macro for impl_180 (impl)
macro_rules! Depcrate_civil_timeimpl_180 {
() => {
// Module: crate::civil::time
// Provides: {"impl_180"}
// Dependencies: {}
# [doc = " Computes the span of time between two times."] # [doc = ""] # [doc = " This will return a negative span when the time being subtracted is greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two times (no rounding and largest units is hours), this will never panic"] # [doc = " or fail in any way."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`Time::since`]."] impl core :: ops :: Sub for Time { type Output = Span ; # [inline] fn sub (self , rhs : Time) -> Span { self . since (rhs) . expect ("since never fails when given Time") } }
};
}
