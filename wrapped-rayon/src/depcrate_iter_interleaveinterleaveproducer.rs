// Generated macro for InterleaveProducer (struct)
macro_rules! Depcrate_iter_interleaveInterleaveProducer {
() => {
// Module: crate::iter::interleave
// Provides: {"InterleaveProducer"}
// Dependencies: {}
struct InterleaveProducer < I , J > where I : Producer , J : Producer < Item = I :: Item > , { i : I , j : J , i_len : usize , j_len : usize , i_next : bool , }
};
}
