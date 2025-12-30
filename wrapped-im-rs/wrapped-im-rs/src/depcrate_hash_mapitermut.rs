// Generated macro for IterMut (struct)
macro_rules! Depcrate_hash_mapIterMut {
() => {
// Module: crate::hash::map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the elements of a map."] pub struct IterMut < 'a , K , V > where K : Clone , V : Clone , { it : NodeIterMut < 'a , (K , V) > , }
};
}
