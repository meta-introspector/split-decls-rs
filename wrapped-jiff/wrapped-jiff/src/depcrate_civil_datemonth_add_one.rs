// Generated macro for month_add_one (function)
macro_rules! Depcrate_civil_datemonth_add_one {
() => {
// Module: crate::civil::date
// Provides: {"month_add_one"}
// Dependencies: {}
# [doc = " Adds or subtracts `sign` from the given `year`/`month`."] # [doc = ""] # [doc = " If month overflows in either direction, then the `year` returned is"] # [doc = " adjusted as appropriate."] fn month_add_one (mut year : Year , mut month : Month , delta : Sign ,) -> Result < (Year , Month) , Error > { month += delta ; if month < C (1) { year -= C (1) ; month += t :: MONTHS_PER_YEAR ; } else if month > t :: MONTHS_PER_YEAR { year += C (1) ; month -= t :: MONTHS_PER_YEAR ; } let year = Year :: try_rfrom ("year" , year) ? ; let month = Month :: try_rfrom ("year" , month) ? ; Ok ((year , month)) }
};
}
