// Generated macro for impl_103 (impl)
macro_rules! Depcrate_civil_datetimeimpl_103 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_103"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: Add < SignedDuration > for DateTime { type Output = DateTime ; # [inline] fn add (self , rhs : SignedDuration) -> DateTime { self . checked_add (rhs) . expect ("adding signed duration to datetime overflowed") } }
};
}
