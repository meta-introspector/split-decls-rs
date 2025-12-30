// Generated macro for RefIter (struct)
macro_rules! Depcrate_baseRefIter {
() => {
// Module: crate::base
// Provides: {"RefIter"}
// Dependencies: {}
# [doc = " An iterator over reference-counted entries of a `SkipList`."] pub struct RefIter < 'a , K , V > { parent : & 'a SkipList < K , V > , head : Option < RefEntry < 'a , K , V > > , tail : Option < RefEntry < 'a , K , V > > , }
};
}
