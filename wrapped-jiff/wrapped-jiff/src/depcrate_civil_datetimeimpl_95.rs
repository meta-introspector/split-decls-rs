// Generated macro for impl_95 (impl)
macro_rules! Depcrate_civil_datetimeimpl_95 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_95"}
// Dependencies: {}
# [doc = " Converts a [`Date`] to a [`DateTime`] with the time set to midnight."] impl From < Date > for DateTime { # [inline] fn from (date : Date) -> DateTime { date . to_datetime (Time :: midnight ()) } }
};
}
