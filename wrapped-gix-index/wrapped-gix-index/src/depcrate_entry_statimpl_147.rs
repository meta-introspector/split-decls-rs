// Generated macro for impl_147 (impl)
macro_rules! Depcrate_entry_statimpl_147 {
() => {
// Module: crate::entry::stat
// Provides: {"impl_147"}
// Dependencies: {}
impl From < Time > for SystemTime { fn from (s : Time) -> Self { std :: time :: UNIX_EPOCH + std :: time :: Duration :: new (s . secs . into () , s . nsecs) } }
};
}
