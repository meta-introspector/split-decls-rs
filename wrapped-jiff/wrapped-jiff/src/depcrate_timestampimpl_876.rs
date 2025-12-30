// Generated macro for impl_876 (impl)
macro_rules! Depcrate_timestampimpl_876 {
() => {
// Module: crate::timestamp
// Provides: {"impl_876"}
// Dependencies: {}
# [doc = " Adds a span of time to a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics when it fails. To handle arithmetic"] # [doc = " without panics, use [`Timestamp::checked_add`]. Note that the failure"] # [doc = " condition includes overflow and using a `Span` with non-zero units greater"] # [doc = " than hours."] impl core :: ops :: Add < Span > for Timestamp { type Output = Timestamp ; # [inline] fn add (self , rhs : Span) -> Timestamp { self . checked_add_span (rhs) . expect ("adding span to timestamp failed") } }
};
}
