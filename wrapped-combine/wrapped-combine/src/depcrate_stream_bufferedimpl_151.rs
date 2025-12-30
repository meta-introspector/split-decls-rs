// Generated macro for impl_151 (impl)
macro_rules! Depcrate_stream_bufferedimpl_151 {
() => {
// Module: crate::stream::buffered
// Provides: {"impl_151"}
// Dependencies: {}
impl < Input > Positioned for Stream < Input > where Input : StreamOnce + Positioned , { # [inline] fn position (& self) -> Self :: Position { if self . offset >= self . buffer_offset { self . iter . position () } else if self . offset < self . buffer_offset - self . buffer . len () { self . buffer . front () . expect ("At least 1 element in the buffer") . 1 . clone () } else { self . buffer [self . buffer . len () - (self . buffer_offset - self . offset)] . 1 . clone () } } }
};
}
