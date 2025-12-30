// Generated macro for impl_9 (impl)
macro_rules! Depcrate_countersimpl_9 {
() => {
// Module: crate::counters
// Provides: {"impl_9"}
// Dependencies: {}
impl WallTime { const NAME : & 'static str = "wall-time" ; pub fn new () -> Self { WallTime { start : Instant :: now () , } } # [inline] fn since_start (& self) -> u64 { self . start . elapsed () . as_nanos () as u64 } }
};
}
