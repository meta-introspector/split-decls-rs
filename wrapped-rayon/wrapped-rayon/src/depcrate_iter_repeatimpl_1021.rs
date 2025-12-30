// Generated macro for impl_1021 (impl)
macro_rules! Depcrate_iter_repeatimpl_1021 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1021"}
// Dependencies: {}
impl < T : Clone > ExactSizeIterator for RepeatNProducer < T > { # [inline] fn len (& self) -> usize { match self { Self :: Repeats (_ , count) => count . get () , Self :: Empty => 0 , } } }
};
}
