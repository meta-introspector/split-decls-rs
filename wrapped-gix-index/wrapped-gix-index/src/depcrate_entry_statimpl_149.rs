// Generated macro for impl_149 (impl)
macro_rules! Depcrate_entry_statimpl_149 {
() => {
// Module: crate::entry::stat
// Provides: {"impl_149"}
// Dependencies: {}
impl From < FileTime > for Time { fn from (value : FileTime) -> Self { Time { secs : value . unix_seconds () . try_into () . expect ("can't represent non-unix times") , nsecs : value . nanoseconds () , } } }
};
}
