// Generated macro for impl_520 (impl)
macro_rules! Depcrate_duration_validated_optionsimpl_520 {
() => {
// Module: crate::duration::validated_options
// Provides: {"impl_520"}
// Dependencies: {}
impl From < DurationFormatterOptions > for ValidatedDurationFormatterOptionsBuilder { fn from (value : DurationFormatterOptions) -> Self { ValidatedDurationFormatterOptionsBuilder { base : value . base , year : value . year . map (FieldStyle :: from) , year_visibility : value . year_visibility , month : value . month . map (FieldStyle :: from) , month_visibility : value . month_visibility , week : value . week . map (FieldStyle :: from) , week_visibility : value . week_visibility , day : value . day . map (FieldStyle :: from) , day_visibility : value . day_visibility , hour : value . hour . map (FieldStyle :: from) , hour_visibility : value . hour_visibility , minute : value . minute . map (FieldStyle :: from) , minute_visibility : value . minute_visibility , second : value . second . map (FieldStyle :: from) , second_visibility : value . second_visibility , millisecond : value . millisecond . map (FieldStyle :: from) , millisecond_visibility : value . millisecond_visibility , microsecond : value . microsecond . map (FieldStyle :: from) , microsecond_visibility : value . microsecond_visibility , nanosecond : value . nanosecond . map (FieldStyle :: from) , nanosecond_visibility : value . nanosecond_visibility , fractional_digits : value . fractional_digits , } } }
};
}
