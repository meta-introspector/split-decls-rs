// Generated macro for impl_378 (impl)
macro_rules! Depcrate_iter_chainimpl_378 {
() => {
// Module: crate::iter::chain
// Provides: {"impl_378"}
// Dependencies: {}
impl < A , B > ChainProducer < A , B > where A : Producer , B : Producer < Item = A :: Item > , { fn new (a_len : usize , a : A , b : B) -> Self { ChainProducer { a_len , a , b } } }
};
}
