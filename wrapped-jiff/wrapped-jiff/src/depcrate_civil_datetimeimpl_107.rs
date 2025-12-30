// Generated macro for impl_107 (impl)
macro_rules! Depcrate_civil_datetimeimpl_107 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_107"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: Add < UnsignedDuration > for DateTime { type Output = DateTime ; # [inline] fn add (self , rhs : UnsignedDuration) -> DateTime { self . checked_add (rhs) . expect ("adding unsigned duration to datetime overflowed") } }
};
}
