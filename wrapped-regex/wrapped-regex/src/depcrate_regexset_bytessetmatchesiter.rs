// Generated macro for SetMatchesIter (struct)
macro_rules! Depcrate_regexset_bytesSetMatchesIter {
() => {
// Module: crate::regexset::bytes
// Provides: {"SetMatchesIter"}
// Dependencies: {}
# [doc = " A borrowed iterator over the set of matches from a regex set."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the [`SetMatches`] value that"] # [doc = " created this iterator."] # [doc = ""] # [doc = " This will always produces matches in ascending order, where the index"] # [doc = " corresponds to the index of the regex that matched with respect to its"] # [doc = " position when initially building the set."] # [doc = ""] # [doc = " This iterator is created by the [`SetMatches::iter`] method."] # [derive (Clone , Debug)] pub struct SetMatchesIter < 'a > (PatternSetIter < 'a >) ;
};
}
