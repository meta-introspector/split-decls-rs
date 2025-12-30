// Generated macro for impl_276 (impl)
macro_rules! Depcrateimpl_276 {
() => {
// Module: crate
// Provides: {"impl_276"}
// Dependencies: {}
impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsRawFd for IoUring < S , C > { fn as_raw_fd (& self) -> RawFd { self . fd . as_raw_fd () } }
};
}
