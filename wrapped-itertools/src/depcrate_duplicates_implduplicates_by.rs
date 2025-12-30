// Generated macro for duplicates_by (function)
macro_rules! Depcrate_duplicates_implduplicates_by {
() => {
// Module: crate::duplicates_impl
// Provides: {"duplicates_by"}
// Dependencies: {}
# [doc = " Create a new `DuplicatesBy` iterator."] pub fn duplicates_by < I , Key , F > (iter : I , f : F) -> DuplicatesBy < I , Key , F > where Key : Eq + Hash , F : FnMut (& I :: Item) -> Key , I : Iterator , { DuplicatesBy :: new (iter , private :: ByFn (f)) }
};
}
