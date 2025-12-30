// Generated macro for impl_3612 (impl)
macro_rules! Depcrate_loops_manual_memcpyimpl_3612 {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"impl_3612"}
// Dependencies: {}
impl Offset { fn negative (value : Sugg < 'static >) -> Self { Self { value : value . into () , sign : OffsetSign :: Negative , } } fn positive (value : Sugg < 'static >) -> Self { Self { value : value . into () , sign : OffsetSign :: Positive , } } fn empty () -> Self { Self :: positive (sugg :: ZERO) } }
};
}
