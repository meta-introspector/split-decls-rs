// Generated macro for WeekdaySet (struct)
macro_rules! Depcrate_weekday_setWeekdaySet {
() => {
// Module: crate::weekday_set
// Provides: {"WeekdaySet"}
// Dependencies: {}
# [doc = " A collection of [`Weekday`]s stored as a single byte."] # [doc = ""] # [doc = " This type is `Copy` and provides efficient set-like and slice-like operations."] # [doc = " Many operations are `const` as well."] # [doc = ""] # [doc = " Implemented as a bitmask where bits 1-7 correspond to Monday-Sunday."] # [derive (Clone , Copy , Default , Hash , PartialEq , Eq , PartialOrd , Ord)] pub struct WeekdaySet (u8) ;
};
}
