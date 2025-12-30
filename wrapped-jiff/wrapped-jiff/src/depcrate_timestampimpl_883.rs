// Generated macro for impl_883 (impl)
macro_rules! Depcrate_timestampimpl_883 {
() => {
// Module: crate::timestamp
// Provides: {"impl_883"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_sub`]."] impl core :: ops :: Sub < SignedDuration > for Timestamp { type Output = Timestamp ; # [inline] fn sub (self , rhs : SignedDuration) -> Timestamp { let rhs = rhs . checked_neg () . expect ("signed duration negation resulted in overflow") ; self . checked_add_duration (rhs) . expect ("subtracting signed duration from timestamp overflowed") } }
};
}
