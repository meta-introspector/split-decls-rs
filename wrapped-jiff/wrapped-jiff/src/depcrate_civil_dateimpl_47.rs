// Generated macro for impl_47 (impl)
macro_rules! Depcrate_civil_dateimpl_47 {
() => {
// Module: crate::civil::date
// Provides: {"impl_47"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: Add < UnsignedDuration > for Date { type Output = Date ; # [inline] fn add (self , rhs : UnsignedDuration) -> Date { self . checked_add (rhs) . expect ("adding unsigned duration to date overflowed") } }
};
}
