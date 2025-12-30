// Generated macro for easter (function)
macro_rules! Depcrate_julianeaster {
() => {
// Module: crate::julian
// Provides: {"easter"}
// Dependencies: {}
# [doc = " Calculates the date of Easter in the given year"] pub fn easter (year : i32) -> RataDie { let shifted_epact = (14 + 11 * year . rem_euclid (19)) % 30 ; let paschal_moon = fixed_from_julian (year , 4 , 19) - shifted_epact as i64 ; k_day_after (0 , paschal_moon) }
};
}
