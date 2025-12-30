// Generated macro for ord_map (function)
macro_rules! Depcrate_proptestord_map {
() => {
// Module: crate::proptest
// Provides: {"ord_map"}
// Dependencies: {}
# [doc = " A strategy for an [`OrdMap`][OrdMap] of a given size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use ::proptest::proptest;"] # [doc = " proptest! {"] # [doc = "     #[test]"] # [doc = "     fn proptest_works(ref m in ord_map(0..9999, \".*\", 10..100)) {"] # [doc = "         assert!(m.len() < 100);"] # [doc = "         assert!(m.len() >= 10);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [OrdMap]: ../struct.OrdMap.html"] pub fn ord_map < K : Strategy + 'static , V : Strategy + 'static > (key : K , value : V , size : Range < usize > ,) -> BoxedStrategy < OrdMap < < K :: Tree as ValueTree > :: Value , < V :: Tree as ValueTree > :: Value > > where < K :: Tree as ValueTree > :: Value : Ord + Clone , < V :: Tree as ValueTree > :: Value : Clone , { :: proptest :: collection :: vec ((key , value) , size . clone ()) . prop_map (OrdMap :: from) . prop_filter ("OrdMap minimum size" . to_owned () , move | m | { m . len () >= size . start }) . boxed () }
};
}
