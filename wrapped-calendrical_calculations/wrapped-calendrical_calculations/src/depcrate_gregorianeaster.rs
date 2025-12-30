// Generated macro for easter (function)
macro_rules! Depcrate_gregorianeaster {
() => {
// Module: crate::gregorian
// Provides: {"easter"}
// Dependencies: {}
# [doc = " Calculates the date of Easter in the given year"] pub fn easter (year : i32) -> RataDie { let century = (year / 100) + 1 ; let shifted_epact = (14 + 11 * year . rem_euclid (19) - century * 3 / 4 + (5 + 8 * century) / 25) . rem_euclid (30) ; let adjusted_epact = shifted_epact + (shifted_epact == 0 || (shifted_epact == 1 && 10 < year . rem_euclid (19))) as i32 ; let paschal_moon = fixed_from_gregorian (year , 4 , 19) - adjusted_epact as i64 ; k_day_after (0 , paschal_moon) }
};
}
