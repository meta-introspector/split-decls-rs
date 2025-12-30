// Generated macro for TryReserveErrorKind (enum)
macro_rules! Depcrate_collectionsTryReserveErrorKind {
() => {
// Module: crate::collections
// Provides: {"TryReserveErrorKind"}
// Dependencies: {}
# [doc = " Details of the allocation that caused a `TryReserveError`"] # [derive (Clone , PartialEq , Eq , Debug)] # [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] # [cfg (not (test))] pub enum TryReserveErrorKind { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " The memory allocator returned an error"] AllocError { # [doc = " The layout of allocation request that failed"] layout : Layout , # [doc (hidden)] # [unstable (feature = "container_error_extra" , issue = "none" , reason = "\
            Enable exposing the allocator’s custom error value \
            if an associated type is added in the future: \
            https://github.com/rust-lang/wg-allocators/issues/23")] non_exhaustive : () , } , }
};
}
