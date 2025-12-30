// Generated macro for impl_43 (impl)
macro_rules! Depcrate_civil_dateimpl_43 {
() => {
// Module: crate::civil::date
// Provides: {"impl_43"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: Add < SignedDuration > for Date { type Output = Date ; # [inline] fn add (self , rhs : SignedDuration) -> Date { self . checked_add (rhs) . expect ("adding signed duration to date overflowed") } }
};
}
