// Generated macro for impl_95 (impl)
macro_rules! Depcrate_util_secondsimpl_95 {
() => {
// Module: crate::util::seconds
// Provides: {"impl_95"}
// Dependencies: {}
impl Seconds { pub (crate) fn from_val (val : & HeaderValue) -> Option < Self > { let secs = val . to_str () . ok () ? . parse () . ok () ? ; Some (Self :: from_secs (secs)) } pub (crate) fn from_secs (secs : u64) -> Self { Self :: from (Duration :: from_secs (secs)) } pub (crate) fn as_u64 (& self) -> u64 { self . 0 . as_secs () } }
};
}
