// Generated macro for new_moon_before (function)
macro_rules! Depcrate_chinese_basednew_moon_before {
() => {
// Module: crate::chinese_based
// Provides: {"new_moon_before"}
// Dependencies: {}
# [doc = " The fixed date in standard time at the observation location of the previous new moon before a given Moment."] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz."] # [doc = " Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5318-L5327"] pub (crate) fn new_moon_before < C : ChineseBased > (moment : Moment) -> RataDie { let new_moon_moment = Astronomical :: new_moon_before (midnight :: < C > (moment)) ; let utc_offset = C :: utc_offset (new_moon_moment . as_rata_die ()) ; (new_moon_moment + utc_offset) . as_rata_die () }
};
}
