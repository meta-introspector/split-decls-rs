// Generated macro for winter_solstice_on_or_before (function)
macro_rules! Depcrate_chinese_basedwinter_solstice_on_or_before {
() => {
// Module: crate::chinese_based
// Provides: {"winter_solstice_on_or_before"}
// Dependencies: {}
# [doc = " Get the fixed date of the nearest winter solstice, in the Chinese time zone,"] # [doc = " on or before a given fixed date."] # [doc = ""] # [doc = " This is valid for several thousand years, but it drifts for large positive"] # [doc = " and negative years. See [`bind_winter_solstice`]."] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz."] # [doc = " Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5359-L5368"] pub (crate) fn winter_solstice_on_or_before < C : ChineseBased > (date : RataDie) -> RataDie { let approx = Astronomical :: estimate_prior_solar_longitude (astronomy :: WINTER , midnight :: < C > ((date + 1) . as_moment ()) ,) ; let mut iters = 0 ; let mut day = Moment :: new ((approx . inner () - 1.0) . floor ()) ; while iters < MAX_ITERS_FOR_MONTHS_OF_YEAR && astronomy :: WINTER >= Astronomical :: solar_longitude (Astronomical :: julian_centuries (midnight :: < C > (day + 1.0 ,))) { iters += 1 ; day += 1.0 ; } debug_assert ! (iters < MAX_ITERS_FOR_MONTHS_OF_YEAR || ! WELL_BEHAVED_ASTRONOMICAL_RANGE . contains (& date) , "Number of iterations was higher than expected") ; day . as_rata_die () }
};
}
