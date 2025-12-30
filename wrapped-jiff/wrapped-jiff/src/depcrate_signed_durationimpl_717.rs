// Generated macro for impl_717 (impl)
macro_rules! Depcrate_signed_durationimpl_717 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_717"}
// Dependencies: {}
impl TryFrom < SignedDuration > for Duration { type Error = Error ; fn try_from (sd : SignedDuration) -> Result < Duration , Error > { if sd . is_negative () { return Err (err ! ("cannot convert negative duration `{sd:?}` to \
                 unsigned `std::time::Duration`" ,)) ; } let secs = u64 :: try_from (sd . as_secs ()) . map_err (| _ | { err ! ("seconds in signed duration {sd:?} overflowed u64") }) ? ; let nanos = u32 :: try_from (sd . subsec_nanos ()) . unwrap () ; Ok (Duration :: new (secs , nanos)) } }
};
}
