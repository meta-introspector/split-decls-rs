// Generated macro for CollectionAllocErr (enum)
macro_rules! Depcrate_collectionsCollectionAllocErr {
() => {
// Module: crate::collections
// Provides: {"CollectionAllocErr"}
// Dependencies: {}
# [doc = " Augments `AllocErr` with a `CapacityOverflow` variant."] # [derive (Clone , PartialEq , Eq , Debug)] pub enum CollectionAllocErr { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " Error due to the allocator (see the documentation for the [`AllocErr`] type)."] AllocErr , }
};
}
