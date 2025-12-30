// Generated macro for OnceLock (struct)
macro_rules! Depcrate_utilitiesOnceLock {
() => {
// Module: crate::utilities
// Provides: {"OnceLock"}
// Dependencies: {}
pub (crate) struct OnceLock < T > { once : Once , value : UnsafeCell < MaybeUninit < T > > , _marker : PhantomData < T > , }
};
}
