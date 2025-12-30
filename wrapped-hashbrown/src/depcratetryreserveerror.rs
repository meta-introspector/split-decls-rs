// Generated macro for TryReserveError (enum)
macro_rules! DepcrateTryReserveError {
() => {
// Module: crate
// Provides: {"TryReserveError"}
// Dependencies: {}
# [doc = " The error type for `try_reserve` methods."] # [derive (Clone , PartialEq , Eq , Debug)] pub enum TryReserveError { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " The memory allocator returned an error"] AllocError { # [doc = " The layout of the allocation request that failed."] layout : alloc :: alloc :: Layout , } , }
};
}
