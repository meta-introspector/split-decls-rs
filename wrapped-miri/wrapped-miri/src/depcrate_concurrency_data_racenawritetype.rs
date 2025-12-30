// Generated macro for NaWriteType (enum)
macro_rules! Depcrate_concurrency_data_raceNaWriteType {
() => {
// Module: crate::concurrency::data_race
// Provides: {"NaWriteType"}
// Dependencies: {}
# [doc = " Type of a non-atomic write operation: allocating memory, non-atomic writes, and"] # [doc = " deallocating memory are all treated as writes for the purpose of the data-race detector."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum NaWriteType { # [doc = " Allocate memory."] Allocate , # [doc = " Standard unsynchronized write."] Write , Retag , # [doc = " Deallocate memory."] # [doc = " Note that when memory is deallocated first, later non-atomic accesses"] # [doc = " will be reported as use-after-free, not as data races."] # [doc = " (Same for `Allocate` above.)"] Deallocate , }
};
}
