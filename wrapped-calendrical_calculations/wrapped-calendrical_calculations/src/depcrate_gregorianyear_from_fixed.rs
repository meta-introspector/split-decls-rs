// Generated macro for year_from_fixed (function)
macro_rules! Depcrate_gregorianyear_from_fixed {
() => {
// Module: crate::gregorian
// Provides: {"year_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1191-L1217>"] pub const fn year_from_fixed (date : RataDie) -> Result < i32 , I32CastError > { let date = date . since (EPOCH) ; let (n_400 , date) = (date . div_euclid (DAYS_IN_400_YEAR_CYCLE) , date . rem_euclid (DAYS_IN_400_YEAR_CYCLE) ,) ; let (n_100 , date) = (date / DAYS_IN_100_YEAR_CYCLE , date % DAYS_IN_100_YEAR_CYCLE) ; let (n_4 , date) = (date / DAYS_IN_4_YEAR_CYCLE , date % DAYS_IN_4_YEAR_CYCLE) ; let n_1 = date / DAYS_IN_YEAR ; let year = 400 * n_400 + 100 * n_100 + 4 * n_4 + n_1 + (n_100 != 4 && n_1 != 4) as i64 ; i64_to_i32 (year) }
};
}
