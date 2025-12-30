// Generated macro for fast_persian_from_fixed (function)
macro_rules! Depcrate_persianfast_persian_from_fixed {
() => {
// Module: crate::persian
// Provides: {"fast_persian_from_fixed"}
// Dependencies: {}
# [doc = " arithmetic_persian_from_fixed, modified to use the 33-year rule method"] pub fn fast_persian_from_fixed (date : RataDie) -> Result < (i32 , u8 , u8) , I32CastError > { let year = fast_persian_year_from_fixed (date) ; let mut year = i64_to_i32 (year) ? ; let mut day_of_year = 1_i64 + (date - fixed_from_fast_persian (year , 1 , 1)) ; if day_of_year == 366 && year >= MIN_NON_LEAP_CORRECTION && NON_LEAP_CORRECTION . binary_search (& year) . is_ok () { year += 1 ; day_of_year = 1 ; } # [allow (unstable_name_collisions)] let month = if day_of_year <= 186 { day_of_year . div_ceil (31) as u8 } else { (day_of_year - 6) . div_ceil (30) as u8 } ; let day = (date - fixed_from_fast_persian (year , month , 1) + 1) as u8 ; Ok ((year , month , day)) }
};
}
