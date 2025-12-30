// Generated macro for PatternIter (struct)
macro_rules! Depcrate_packed_patternPatternIter {
() => {
// Module: crate::packed::pattern
// Provides: {"PatternIter"}
// Dependencies: {}
# [doc = " An iterator over the patterns in the `Patterns` collection."] # [doc = ""] # [doc = " The order of the patterns provided by this iterator is consistent with the"] # [doc = " match semantics of the originating collection of patterns."] # [doc = ""] # [doc = " The lifetime `'p` corresponds to the lifetime of the collection of patterns"] # [doc = " this is iterating over."] # [derive (Debug)] pub (crate) struct PatternIter < 'p > { patterns : & 'p Patterns , i : usize , }
};
}
