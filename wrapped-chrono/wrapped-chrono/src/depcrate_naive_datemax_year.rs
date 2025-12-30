// Generated macro for MAX_YEAR (const)
macro_rules! Depcrate_naive_dateMAX_YEAR {
() => {
// Module: crate::naive::date
// Provides: {"MAX_YEAR"}
// Dependencies: {}
# [doc = " MAX_YEAR is one year less than the type is capable of representing. Internally we may sometimes"] # [doc = " use the headroom, notably to handle cases where the offset of a `DateTime` constructed with"] # [doc = " `NaiveDate::MAX` pushes it beyond the valid, representable range."] pub (super) const MAX_YEAR : i32 = (i32 :: MAX >> 13) - 1 ;
};
}
