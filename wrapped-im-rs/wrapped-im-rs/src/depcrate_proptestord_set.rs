// Generated macro for ord_set (function)
macro_rules! Depcrate_proptestord_set {
() => {
// Module: crate::proptest
// Provides: {"ord_set"}
// Dependencies: {}
# [doc = " A strategy for an [`OrdSet`][OrdSet] of a given size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use ::proptest::proptest;"] # [doc = " proptest! {"] # [doc = "     #[test]"] # [doc = "     fn proptest_a_set(ref s in ord_set(\".*\", 10..100)) {"] # [doc = "         assert!(s.len() < 100);"] # [doc = "         assert!(s.len() >= 10);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [OrdSet]: ../struct.OrdSet.html"] pub fn ord_set < A : Strategy + 'static > (element : A , size : Range < usize > ,) -> BoxedStrategy < OrdSet < < A :: Tree as ValueTree > :: Value > > where < A :: Tree as ValueTree > :: Value : Ord + Clone , { :: proptest :: collection :: vec (element , size . clone ()) . prop_map (OrdSet :: from) . prop_filter ("OrdSet minimum size" . to_owned () , move | s | { s . len () >= size . start }) . boxed () }
};
}
