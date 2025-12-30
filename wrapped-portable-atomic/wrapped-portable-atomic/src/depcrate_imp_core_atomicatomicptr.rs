// Generated macro for AtomicPtr (struct)
macro_rules! Depcrate_imp_core_atomicAtomicPtr {
() => {
// Module: crate::imp::core_atomic
// Provides: {"AtomicPtr"}
// Dependencies: {}
# [repr (transparent)] pub (crate) struct AtomicPtr < T > { inner : core :: sync :: atomic :: AtomicPtr < T > , _not_ref_unwind_safe : PhantomData < NotRefUnwindSafe > , }
};
}
