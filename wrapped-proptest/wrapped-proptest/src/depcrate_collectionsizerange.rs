// Generated macro for SizeRange (struct)
macro_rules! Depcrate_collectionSizeRange {
() => {
// Module: crate::collection
// Provides: {"SizeRange"}
// Dependencies: {}
# [doc = " The minimum and maximum range/bounds on the size of a collection."] # [doc = " The interval must form a subset of `[0, std::usize::MAX)`."] # [doc = ""] # [doc = " A value like `0..=std::usize::MAX` will still be accepted but will silently"] # [doc = " truncate the maximum to `std::usize::MAX - 1`."] # [doc = ""] # [doc = " The `Default` is `0..PROPTEST_MAX_DEFAULT_SIZE_RANGE`. The max can be set with"] # [doc = " the `PROPTEST_MAX_DEFAULT_SIZE_RANGE` env var, which defaults to `100`."] # [derive (Clone , PartialEq , Eq , Hash , Debug)] pub struct SizeRange (Range < usize >) ;
};
}
