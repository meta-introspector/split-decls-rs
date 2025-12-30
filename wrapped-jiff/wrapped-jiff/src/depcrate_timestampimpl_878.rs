// Generated macro for impl_878 (impl)
macro_rules! Depcrate_timestampimpl_878 {
() => {
// Module: crate::timestamp
// Provides: {"impl_878"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics when it fails. To handle arithmetic"] # [doc = " without panics, use [`Timestamp::checked_sub`]. Note that the failure"] # [doc = " condition includes overflow and using a `Span` with non-zero units greater"] # [doc = " than hours."] impl core :: ops :: Sub < Span > for Timestamp { type Output = Timestamp ; # [inline] fn sub (self , rhs : Span) -> Timestamp { self . checked_add_span (rhs . negate ()) . expect ("subtracting span from timestamp failed") } }
};
}
