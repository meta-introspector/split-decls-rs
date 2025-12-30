// Generated macro for check_date_validity (function)
macro_rules! Depcrate_parsers_datetimecheck_date_validity {
() => {
// Module: crate::parsers::datetime
// Provides: {"check_date_validity"}
// Dependencies: {}
# [inline] fn check_date_validity (year : i32 , month : u8 , day : u8) -> ParserResult < () > { let Some (days_in_month) = days_in_month (year , month) else { return Err (ParseError :: InvalidMonthRange) ; } ; if ! (1 ..= days_in_month) . contains (& day) { return Err (ParseError :: InvalidDayRange) ; } Ok (()) }
};
}
