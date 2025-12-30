// Generated macro for impl_32 (impl)
macro_rules! Depcrate_counterimpl_32 {
() => {
// Module: crate::counter
// Provides: {"impl_32"}
// Dependencies: {}
impl < W > CounterWriter < W > { # [doc = " Returns the total number of bytes written since construction or the"] # [doc = " last time `reset` was called."] # [inline] pub (crate) fn count (& self) -> u64 { self . count } # [doc = " Returns the total number of bytes written since construction."] # [inline] pub (crate) fn total_count (& self) -> u64 { self . total_count + self . count } # [doc = " Resets the number of bytes written to `0`."] # [inline] pub (crate) fn reset_count (& mut self) { self . total_count += self . count ; self . count = 0 ; } # [inline] pub (crate) fn get_mut (& mut self) -> & mut W { & mut self . wtr } # [inline] pub (crate) fn into_inner (self) -> W { self . wtr } }
};
}
