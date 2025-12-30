// Generated macro for impl_881 (impl)
macro_rules! Depcrate_timestampimpl_881 {
() => {
// Module: crate::timestamp
// Provides: {"impl_881"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_add`]."] impl core :: ops :: Add < SignedDuration > for Timestamp { type Output = Timestamp ; # [inline] fn add (self , rhs : SignedDuration) -> Timestamp { self . checked_add_duration (rhs) . expect ("adding signed duration to timestamp overflowed") } }
};
}
