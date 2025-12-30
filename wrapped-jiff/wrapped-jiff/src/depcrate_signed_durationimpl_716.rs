// Generated macro for impl_716 (impl)
macro_rules! Depcrate_signed_durationimpl_716 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_716"}
// Dependencies: {}
impl TryFrom < Duration > for SignedDuration { type Error = Error ; fn try_from (d : Duration) -> Result < SignedDuration , Error > { let secs = i64 :: try_from (d . as_secs ()) . map_err (| _ | { err ! ("seconds in unsigned duration {d:?} overflowed i64") }) ? ; let nanos = i32 :: try_from (d . subsec_nanos ()) . unwrap () ; Ok (SignedDuration :: new_unchecked (secs , nanos)) } }
};
}
