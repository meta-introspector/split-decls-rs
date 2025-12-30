// Generated macro for impl_879 (impl)
macro_rules! Depcrate_timestampimpl_879 {
() => {
// Module: crate::timestamp
// Provides: {"impl_879"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics when it fails. To handle arithmetic"] # [doc = " without panics, use [`Timestamp::checked_sub`]. Note that the failure"] # [doc = " condition includes overflow and using a `Span` with non-zero units greater"] # [doc = " than hours."] impl core :: ops :: SubAssign < Span > for Timestamp { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = * self - rhs } }
};
}
