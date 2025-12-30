// Generated macro for impl_105 (impl)
macro_rules! Depcrate_civil_datetimeimpl_105 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_105"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: Sub < SignedDuration > for DateTime { type Output = DateTime ; # [inline] fn sub (self , rhs : SignedDuration) -> DateTime { self . checked_sub (rhs) . expect ("subtracting signed duration from datetime overflowed") } }
};
}
