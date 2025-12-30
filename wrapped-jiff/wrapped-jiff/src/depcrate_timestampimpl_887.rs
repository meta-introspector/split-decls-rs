// Generated macro for impl_887 (impl)
macro_rules! Depcrate_timestampimpl_887 {
() => {
// Module: crate::timestamp
// Provides: {"impl_887"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a timestamp."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_sub`]."] impl core :: ops :: Sub < UnsignedDuration > for Timestamp { type Output = Timestamp ; # [inline] fn sub (self , rhs : UnsignedDuration) -> Timestamp { self . checked_sub (rhs) . expect ("subtracting unsigned duration from timestamp overflowed") } }
};
}
