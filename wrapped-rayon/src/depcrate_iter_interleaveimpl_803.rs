// Generated macro for impl_803 (impl)
macro_rules! Depcrate_iter_interleaveimpl_803 {
() => {
// Module: crate::iter::interleave
// Provides: {"impl_803"}
// Dependencies: {}
impl < I , J > ExactSizeIterator for InterleaveSeq < I , J > where I : ExactSizeIterator , J : ExactSizeIterator < Item = I :: Item > , { # [inline] fn len (& self) -> usize { self . i . len () + self . j . len () } }
};
}
