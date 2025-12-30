// Generated macro for CollectionAllocErr (enum)
macro_rules! DepcrateCollectionAllocErr {
() => {
// Module: crate
// Provides: {"CollectionAllocErr"}
// Dependencies: {}
# [doc = " Error type for APIs with fallible heap allocation"] # [derive (Debug)] pub enum CollectionAllocErr { # [doc = " Overflow `usize::MAX` or other error during size computation"] CapacityOverflow , # [doc = " The allocator return an error"] AllocErr { # [doc = " The layout that was passed to the allocator"] layout : Layout , } , }
};
}
