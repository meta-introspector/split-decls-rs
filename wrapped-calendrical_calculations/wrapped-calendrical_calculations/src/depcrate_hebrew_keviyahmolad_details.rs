// Generated macro for molad_details (function)
macro_rules! Depcrate_hebrew_keviyahmolad_details {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"molad_details"}
// Dependencies: {}
# [doc = " Given a Hebrew Year, returns its molad specified as:"] # [doc = ""] # [doc = " - The number of weeks since the week of Beharad (Oct 6, 3761 BCE Julian)"] # [doc = " - The number of ḥalakim since the start of the week (Hebrew Sunday, starting on Saturday at 18:00)"] # [inline] fn molad_details (h_year : i32) -> (i64 , i32) { let months_preceding = months_preceding_molad (h_year) ; let molad = MOLAD_BEHERAD_OFFSET as i64 + months_preceding * HEBREW_LUNATION_TIME as i64 ; let weeks_since_beharad = molad . div_euclid (ḤALAKIM_IN_WEEK) ; let in_week = molad . rem_euclid (ḤALAKIM_IN_WEEK) ; let in_week = i32 :: try_from (in_week) ; debug_assert ! (in_week . is_ok () , "ḤALAKIM_IN_WEEK should fit in an i32") ; (weeks_since_beharad , in_week . unwrap_or (0)) }
};
}
