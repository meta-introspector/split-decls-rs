// Generated macro for impl_521 (impl)
macro_rules! Depcrate_duration_validated_optionsimpl_521 {
() => {
// Module: crate::duration::validated_options
// Provides: {"impl_521"}
// Dependencies: {}
impl TryFrom < ValidatedDurationFormatterOptionsBuilder > for ValidatedDurationFormatterOptions { type Error = () ; fn try_from (value : ValidatedDurationFormatterOptionsBuilder) -> Result < Self , Self :: Error > { Ok (ValidatedDurationFormatterOptions { base : value . base , year : value . year . ok_or (()) ? , year_visibility : value . year_visibility . ok_or (()) ? , month : value . month . ok_or (()) ? , month_visibility : value . month_visibility . ok_or (()) ? , week : value . week . ok_or (()) ? , week_visibility : value . week_visibility . ok_or (()) ? , day : value . day . ok_or (()) ? , day_visibility : value . day_visibility . ok_or (()) ? , hour : value . hour . ok_or (()) ? , hour_visibility : value . hour_visibility . ok_or (()) ? , minute : value . minute . ok_or (()) ? , minute_visibility : value . minute_visibility . ok_or (()) ? , second : value . second . ok_or (()) ? , second_visibility : value . second_visibility . ok_or (()) ? , millisecond : value . millisecond . ok_or (()) ? , millisecond_visibility : value . millisecond_visibility . ok_or (()) ? , microsecond : value . microsecond . ok_or (()) ? , microsecond_visibility : value . microsecond_visibility . ok_or (()) ? , nanosecond : value . nanosecond . ok_or (()) ? , nanosecond_visibility : value . nanosecond_visibility . ok_or (()) ? , fractional_digits : value . fractional_digits , }) } }
};
}
