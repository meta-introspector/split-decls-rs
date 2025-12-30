// Generated macro for Iter (struct)
macro_rules! Depcrate_baseIter {
() => {
// Module: crate::base
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `SkipList`."] pub struct Iter < 'a : 'g , 'g , K , V > { parent : & 'a SkipList < K , V > , head : Option < & 'g Node < K , V > > , tail : Option < & 'g Node < K , V > > , guard : & 'g Guard , }
};
}
