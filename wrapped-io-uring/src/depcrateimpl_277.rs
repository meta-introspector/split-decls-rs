// Generated macro for impl_277 (impl)
macro_rules! Depcrateimpl_277 {
() => {
// Module: crate
// Provides: {"impl_277"}
// Dependencies: {}
# [cfg (feature = "io_safety")] impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsFd for IoUring < S , C > { fn as_fd (& self) -> BorrowedFd < '_ > { self . fd . as_fd () } }
};
}
