// Generated macro for day_before_year (function)
macro_rules! Depcrate_julianday_before_year {
() => {
// Module: crate::julian
// Provides: {"day_before_year"}
// Dependencies: {}
# [doc = " Calculates the day before Jan 1 of `year`."] pub const fn day_before_year (year : i32) -> RataDie { let prev_year = (year as i64) - 1 ; let mut fixed : i64 = DAYS_IN_YEAR * prev_year ; const YEAR_SHIFT : i64 = (- (i32 :: MIN as i64 - 1) / 4 + 1) * 4 ; fixed += (prev_year + YEAR_SHIFT) / 4 - const { YEAR_SHIFT / 4 } ; JULIAN_EPOCH . add (fixed - 1) }
};
}
