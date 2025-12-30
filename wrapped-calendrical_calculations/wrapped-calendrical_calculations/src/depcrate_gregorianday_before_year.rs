// Generated macro for day_before_year (function)
macro_rules! Depcrate_gregorianday_before_year {
() => {
// Module: crate::gregorian
// Provides: {"day_before_year"}
// Dependencies: {}
# [doc = " Calculates the day before Jan 1 of `year`."] pub const fn day_before_year (year : i32) -> RataDie { let prev_year = (year as i64) - 1 ; let mut fixed : i64 = DAYS_IN_YEAR * prev_year ; const YEAR_SHIFT : i64 = (- (i32 :: MIN as i64 - 1) / 400 + 1) * 400 ; fixed += (prev_year + YEAR_SHIFT) / 4 - (prev_year + YEAR_SHIFT) / 100 + (prev_year + YEAR_SHIFT) / 400 - const { YEAR_SHIFT / 4 - YEAR_SHIFT / 100 + YEAR_SHIFT / 400 } ; EPOCH . add (fixed - 1) }
};
}
