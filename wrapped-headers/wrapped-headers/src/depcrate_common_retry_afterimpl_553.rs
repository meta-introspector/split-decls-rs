// Generated macro for impl_553 (impl)
macro_rules! Depcrate_common_retry_afterimpl_553 {
() => {
// Module: crate::common::retry_after
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'a > From < & 'a After > for HeaderValue { fn from (after : & 'a After) -> HeaderValue { match * after { After :: Delay (ref delay) => delay . into () , After :: DateTime (ref date) => date . into () , } } }
};
}
