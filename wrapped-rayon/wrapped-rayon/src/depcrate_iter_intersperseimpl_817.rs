// Generated macro for impl_817 (impl)
macro_rules! Depcrate_iter_intersperseimpl_817 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_817"}
// Dependencies: {}
impl < I > Intersperse < I > where I : ParallelIterator < Item : Clone > , { # [doc = " Creates a new `Intersperse` iterator"] pub (super) fn new (base : I , item : I :: Item) -> Self { Intersperse { base , item } } }
};
}
