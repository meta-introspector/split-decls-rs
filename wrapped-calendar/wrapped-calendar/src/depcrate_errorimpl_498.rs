// Generated macro for impl_498 (impl)
macro_rules! Depcrate_errorimpl_498 {
() => {
// Module: crate::error
// Provides: {"impl_498"}
// Dependencies: {}
impl From < EcmaReferenceYearError > for DateFromFieldsError { # [inline] fn from (value : EcmaReferenceYearError) -> Self { match value { EcmaReferenceYearError :: Unimplemented => DateFromFieldsError :: NotEnoughFields , EcmaReferenceYearError :: MonthCodeNotInCalendar => { DateFromFieldsError :: MonthCodeNotInCalendar } } } }
};
}
