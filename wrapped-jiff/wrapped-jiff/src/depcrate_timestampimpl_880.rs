// Generated macro for impl_880 (impl)
macro_rules! Depcrate_timestampimpl_880 {
() => {
// Module: crate::timestamp
// Provides: {"impl_880"}
// Dependencies: {}
# [doc = " Computes the span of time between two timestamps."] # [doc = ""] # [doc = " This will return a negative span when the timestamp being subtracted is"] # [doc = " greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two timestamps (no rounding and largest units is seconds), this will never"] # [doc = " panic or fail in any way."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`Timestamp::since`]."] impl core :: ops :: Sub for Timestamp { type Output = Span ; # [inline] fn sub (self , rhs : Timestamp) -> Span { self . since (rhs) . expect ("since never fails when given Timestamp") } }
};
}
