// Generated macro for OnceRef (struct)
macro_rules! Depcrate_raceOnceRef {
() => {
// Module: crate::race
// Provides: {"OnceRef"}
// Dependencies: {}
# [doc = " A thread-safe cell which can be written to only once."] pub struct OnceRef < 'a , T > { inner : AtomicPtr < T > , ghost : PhantomData < UnsafeCell < & 'a T > > , }
};
}
