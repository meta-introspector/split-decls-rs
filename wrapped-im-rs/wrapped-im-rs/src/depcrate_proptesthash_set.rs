// Generated macro for hash_set (function)
macro_rules! Depcrate_proptesthash_set {
() => {
// Module: crate::proptest
// Provides: {"hash_set"}
// Dependencies: {}
# [doc = " A strategy for a [`HashSet`][HashSet] of a given size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use ::proptest::proptest;"] # [doc = " proptest! {"] # [doc = "     #[test]"] # [doc = "     fn proptest_a_set(ref s in hash_set(\".*\", 10..100)) {"] # [doc = "         assert!(s.len() < 100);"] # [doc = "         assert!(s.len() >= 10);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [HashSet]: ../struct.HashSet.html"] pub fn hash_set < A : Strategy + 'static > (element : A , size : Range < usize > ,) -> BoxedStrategy < HashSet < < A :: Tree as ValueTree > :: Value > > where < A :: Tree as ValueTree > :: Value : Hash + Eq + Clone , { :: proptest :: collection :: vec (element , size . clone ()) . prop_map (HashSet :: from) . prop_filter ("HashSet minimum size" . to_owned () , move | s | { s . len () >= size . start }) . boxed () }
};
}
