// Generated macro for impl_42 (impl)
macro_rules! Depcrate_civil_dateimpl_42 {
() => {
// Module: crate::civil::date
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " Computes the span of time between two dates."] # [doc = ""] # [doc = " This will return a negative span when the date being subtracted is greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two date (no rounding and largest units is days), this will never panic or"] # [doc = " fail in any way."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`Date::since`]."] impl core :: ops :: Sub for Date { type Output = Span ; # [inline] fn sub (self , rhs : Date) -> Span { self . since (rhs) . expect ("since never fails when given Date") } }
};
}
