// Generated macro for VClockAlloc (struct)
macro_rules! Depcrate_concurrency_data_raceVClockAlloc {
() => {
// Module: crate::concurrency::data_race
// Provides: {"VClockAlloc"}
// Dependencies: {}
# [doc = " Vector clock metadata for a logical memory allocation."] # [derive (Debug , Clone)] pub struct VClockAlloc { # [doc = " Assigning each byte a MemoryCellClocks."] alloc_ranges : RefCell < DedupRangeMap < MemoryCellClocks > > , }
};
}
