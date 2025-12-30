// Generated macro for check_extrema (function)
macro_rules! Depcrate_tests_extremacheck_extrema {
() => {
// Module: crate::tests::extrema
// Provides: {"check_extrema"}
// Dependencies: {}
# [track_caller] fn check_extrema < C : Calendar > (cal : C) { let min_date_iso = Date :: try_new_iso (- 271821 , 4 , 19) . unwrap () ; let max_date_iso = Date :: try_new_iso (275760 , 9 , 13) . unwrap () ; let min_date = min_date_iso . to_calendar (Ref (& cal)) ; let max_date = max_date_iso . to_calendar (Ref (& cal)) ; println ! ("min.year = {:?}, max.year = {:?} (cal = {})" , min_date . year () , max_date . year () , cal . debug_name ()) ; }
};
}
