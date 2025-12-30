// Generated macro for impl_calendar_period_marker (macro)
macro_rules! Depcrate_fieldsetsimpl_calendar_period_marker {
() => {
// Module: crate::fieldsets
// Provides: {"impl_calendar_period_marker"}
// Dependencies: {}
# [doc = " Implements a field set of calendar period fields."] # [doc = ""] # [doc = " Several arguments to this macro are required, and the rest are optional."] # [doc = " The optional arguments should be written as `key = yes,` if that parameter"] # [doc = " should be included."] # [doc = ""] # [doc = " See [`impl_date_marker`]."] macro_rules ! impl_calendar_period_marker { ($ (# [$ attr : meta]) * $ type : ident , description = $ description : literal , sample_length = $ sample_length : ident , sample = $ sample : literal , $ (years = $ years_yes : ident ,) ? $ (months = $ months_yes : ident ,) ? $ (dates = $ dates_yes : ident ,) ? $ (input_year = $ year_yes : ident ,) ? $ (input_month = $ month_yes : ident ,) ? $ (input_any_calendar_kind = $ any_calendar_kind_yes : ident ,) ? $ (option_alignment = $ option_alignment_yes : ident ,) ?) => { impl_date_or_calendar_period_marker ! ($ (# [$ attr]) * $ type , description = $ description , sample_length = $ sample_length , sample = $ sample , $ (years = $ years_yes ,) ? $ (months = $ months_yes ,) ? $ (dates = $ dates_yes ,) ? $ (input_year = $ year_yes ,) ? $ (input_month = $ month_yes ,) ? $ (input_any_calendar_kind = $ any_calendar_kind_yes ,) ? $ (option_alignment = $ option_alignment_yes ,) ?) ; impl_composite ! ($ type , CalendarPeriod , CalendarPeriodFieldSet) ; } ; }
};
}
