// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_tableExtractIf {
() => {
// Module: crate::table
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " A draining iterator over entries of a `HashTable` which don't satisfy the predicate `f`."] # [doc = ""] # [doc = " This `struct` is created by [`HashTable::extract_if`]. See its"] # [doc = " documentation for more."] # [must_use = "Iterators are lazy unless consumed"] pub struct ExtractIf < 'a , T , F , A : Allocator = Global > { f : F , inner : RawExtractIf < 'a , T , A > , }
};
}
