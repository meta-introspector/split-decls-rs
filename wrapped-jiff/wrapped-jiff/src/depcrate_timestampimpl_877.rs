// Generated macro for impl_877 (impl)
macro_rules! Depcrate_timestampimpl_877 {
() => {
// Module: crate::timestamp
// Provides: {"impl_877"}
// Dependencies: {}
# [doc = " Adds a span of time to a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics when it fails. To handle arithmetic"] # [doc = " without panics, use [`Timestamp::checked_add`]. Note that the failure"] # [doc = " condition includes overflow and using a `Span` with non-zero units greater"] # [doc = " than hours."] impl core :: ops :: AddAssign < Span > for Timestamp { # [inline] fn add_assign (& mut self , rhs : Span) { * self = * self + rhs } }
};
}
