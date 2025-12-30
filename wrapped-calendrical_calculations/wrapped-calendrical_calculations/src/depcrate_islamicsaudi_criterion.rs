// Generated macro for saudi_criterion (function)
macro_rules! Depcrate_islamicsaudi_criterion {
() => {
// Module: crate::islamic
// Provides: {"saudi_criterion"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6957>"] fn saudi_criterion (date : RataDie) -> Option < bool > { let sunset = Astronomical :: sunset ((date - 1) . as_moment () , MECCA) ? ; let tee = Location :: universal_from_standard (sunset , MECCA) ; let phase = Astronomical :: lunar_phase (tee , Astronomical :: julian_centuries (tee)) ; let moonlag = Astronomical :: moonlag ((date - 1) . as_moment () , MECCA) ? ; Some (phase > 0.0 && phase < 90.0 && moonlag > 0.0) }
};
}
