// Generated macro for AtomicRef (struct)
macro_rules! Depcrate_atomic_refAtomicRef {
() => {
// Module: crate::atomic_ref
// Provides: {"AtomicRef"}
// Dependencies: {}
# [doc = " This is essentially an `AtomicPtr` but is guaranteed to always be valid"] pub struct AtomicRef < T : 'static > (AtomicPtr < T > , PhantomData < & 'static T >) ;
};
}
