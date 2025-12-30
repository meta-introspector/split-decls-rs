// Generated macro for impl_885 (impl)
macro_rules! Depcrate_timestampimpl_885 {
() => {
// Module: crate::timestamp
// Provides: {"impl_885"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_add`]."] impl core :: ops :: Add < UnsignedDuration > for Timestamp { type Output = Timestamp ; # [inline] fn add (self , rhs : UnsignedDuration) -> Timestamp { self . checked_add (rhs) . expect ("adding unsigned duration to timestamp overflowed") } }
};
}
