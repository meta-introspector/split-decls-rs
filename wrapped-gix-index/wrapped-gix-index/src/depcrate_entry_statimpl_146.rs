// Generated macro for impl_146 (impl)
macro_rules! Depcrate_entry_statimpl_146 {
() => {
// Module: crate::entry::stat
// Provides: {"impl_146"}
// Dependencies: {}
impl TryFrom < SystemTime > for Time { type Error = SystemTimeError ; fn try_from (s : SystemTime) -> Result < Self , SystemTimeError > { let d = s . duration_since (std :: time :: UNIX_EPOCH) ? ; Ok (Time { secs : d . as_secs () as u32 , nsecs : d . subsec_nanos () , }) } }
};
}
