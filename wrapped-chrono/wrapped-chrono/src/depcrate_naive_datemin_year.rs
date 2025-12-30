// Generated macro for MIN_YEAR (const)
macro_rules! Depcrate_naive_dateMIN_YEAR {
() => {
// Module: crate::naive::date
// Provides: {"MIN_YEAR"}
// Dependencies: {}
# [doc = " MIN_YEAR is one year more than the type is capable of representing. Internally we may sometimes"] # [doc = " use the headroom, notably to handle cases where the offset of a `DateTime` constructed with"] # [doc = " `NaiveDate::MIN` pushes it beyond the valid, representable range."] pub (super) const MIN_YEAR : i32 = (i32 :: MIN >> 13) + 1 ;
};
}
