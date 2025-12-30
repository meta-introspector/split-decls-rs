// Generated macro for impl_11 (impl)
macro_rules! Depcrate_acquireimpl_11 {
() => {
// Module: crate::acquire
// Provides: {"impl_11"}
// Dependencies: {}
impl From < Duration > for Fail { fn from (value : Duration) -> Self { if value . is_zero () { Fail :: Immediately } else { Fail :: AfterDurationWithBackoff (value) } } }
};
}
