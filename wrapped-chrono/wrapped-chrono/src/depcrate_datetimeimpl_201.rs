// Generated macro for impl_201 (impl)
macro_rules! Depcrate_datetimeimpl_201 {
() => {
// Module: crate::datetime
// Provides: {"impl_201"}
// Dependencies: {}
impl Default for DateTime < FixedOffset > { fn default () -> Self { FixedOffset :: west_opt (0) . unwrap () . from_utc_datetime (& NaiveDateTime :: default ()) } }
};
}
