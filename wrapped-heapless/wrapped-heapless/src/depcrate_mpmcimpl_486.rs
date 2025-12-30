// Generated macro for impl_486 (impl)
macro_rules! Depcrate_mpmcimpl_486 {
() => {
// Module: crate::mpmc
// Provides: {"impl_486"}
// Dependencies: {}
impl < T > Cell < T > { const fn new (seq : usize) -> Self { Self { data : MaybeUninit :: uninit () , sequence : AtomicTargetSize :: new (seq as UintSize) , } } }
};
}
