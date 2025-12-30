// Generated macro for TryReserveErrorKind (enum)
macro_rules! Depcrate_raw_vecTryReserveErrorKind {
() => {
// Module: crate::raw_vec
// Provides: {"TryReserveErrorKind"}
// Dependencies: {}
# [doc = " Details of the allocation that caused a `TryReserveError`"] # [derive (Clone , PartialEq , Eq , Debug)] pub enum TryReserveErrorKind { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " The memory allocator returned an error"] AllocError { # [doc = " The layout of allocation request that failed"] layout : Layout , # [doc (hidden)] non_exhaustive : () , } , }
};
}
