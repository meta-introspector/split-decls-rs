// Generated macro for vector (function)
macro_rules! Depcrate_proptestvector {
() => {
// Module: crate::proptest
// Provides: {"vector"}
// Dependencies: {}
# [doc = " A strategy for generating a [`Vector`][Vector] of a certain size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use ::proptest::proptest;"] # [doc = " proptest! {"] # [doc = "     #[test]"] # [doc = "     fn proptest_a_vector(ref l in vector(\".*\", 10..100)) {"] # [doc = "         assert!(l.len() < 100);"] # [doc = "         assert!(l.len() >= 10);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [Vector]: ../struct.Vector.html"] pub fn vector < A : Strategy + 'static > (element : A , size : Range < usize > ,) -> BoxedStrategy < Vector < < A :: Tree as ValueTree > :: Value > > where < A :: Tree as ValueTree > :: Value : Clone , { vec (element , size) . prop_map (Vector :: from_iter) . boxed () }
};
}
