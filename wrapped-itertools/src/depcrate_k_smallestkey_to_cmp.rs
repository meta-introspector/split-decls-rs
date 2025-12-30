// Generated macro for key_to_cmp (function)
macro_rules! Depcrate_k_smallestkey_to_cmp {
() => {
// Module: crate::k_smallest
// Provides: {"key_to_cmp"}
// Dependencies: {}
# [inline] pub (crate) fn key_to_cmp < T , K , F > (mut key : F) -> impl FnMut (& T , & T) -> Ordering where F : FnMut (& T) -> K , K : Ord , { move | a , b | key (a) . cmp (& key (b)) }
};
}
