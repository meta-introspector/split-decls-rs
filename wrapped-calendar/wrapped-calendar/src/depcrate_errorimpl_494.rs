// Generated macro for impl_494 (impl)
macro_rules! Depcrate_errorimpl_494 {
() => {
// Module: crate::error
// Provides: {"impl_494"}
// Dependencies: {}
impl From < MonthCodeError > for DateFromFieldsError { # [inline] fn from (value : MonthCodeError) -> Self { match value { MonthCodeError :: NotInCalendar => DateFromFieldsError :: MonthCodeNotInCalendar , MonthCodeError :: NotInYear => DateFromFieldsError :: MonthCodeNotInYear , } } }
};
}
