// Generated macro for hash_map (function)
macro_rules! Depcrate_proptesthash_map {
() => {
// Module: crate::proptest
// Provides: {"hash_map"}
// Dependencies: {}
# [doc = " A strategy for a [`HashMap`][HashMap] of a given size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use ::proptest::proptest;"] # [doc = " proptest! {"] # [doc = "     #[test]"] # [doc = "     fn proptest_works(ref m in hash_map(0..9999, \".*\", 10..100)) {"] # [doc = "         assert!(m.len() < 100);"] # [doc = "         assert!(m.len() >= 10);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [HashMap]: ../struct.HashMap.html"] pub fn hash_map < K : Strategy + 'static , V : Strategy + 'static > (key : K , value : V , size : Range < usize > ,) -> BoxedStrategy < HashMap < < K :: Tree as ValueTree > :: Value , < V :: Tree as ValueTree > :: Value > > where < K :: Tree as ValueTree > :: Value : Hash + Eq + Clone , < V :: Tree as ValueTree > :: Value : Clone , { :: proptest :: collection :: vec ((key , value) , size . clone ()) . prop_map (HashMap :: from) . prop_filter ("Map minimum size" . to_owned () , move | m | { m . len () >= size . start }) . boxed () }
};
}
