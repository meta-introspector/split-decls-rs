// Generated macro for impl_492 (impl)
macro_rules! Depcrate_errorimpl_492 {
() => {
// Module: crate::error
// Provides: {"impl_492"}
// Dependencies: {}
impl From < MonthCodeParseError > for DateFromFieldsError { # [inline] fn from (value : MonthCodeParseError) -> Self { match value { MonthCodeParseError :: InvalidSyntax => DateFromFieldsError :: MonthCodeInvalidSyntax , } } }
};
}
