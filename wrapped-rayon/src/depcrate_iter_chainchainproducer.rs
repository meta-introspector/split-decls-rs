// Generated macro for ChainProducer (struct)
macro_rules! Depcrate_iter_chainChainProducer {
() => {
// Module: crate::iter::chain
// Provides: {"ChainProducer"}
// Dependencies: {}
struct ChainProducer < A , B > where A : Producer , B : Producer < Item = A :: Item > , { a_len : usize , a : A , b : B , }
};
}
