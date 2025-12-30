// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_setExtractIf {
() => {
// Module: crate::set
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " A draining iterator over entries of a `HashSet` which don't satisfy the predicate `f`."] # [doc = ""] # [doc = " This `struct` is created by the [`extract_if`] method on [`HashSet`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`extract_if`]: struct.HashSet.html#method.extract_if"] # [doc = " [`HashSet`]: struct.HashSet.html"] # [must_use = "Iterators are lazy unless consumed"] pub struct ExtractIf < 'a , K , F , A : Allocator = Global > { f : F , inner : RawExtractIf < 'a , (K , ()) , A > , }
};
}
