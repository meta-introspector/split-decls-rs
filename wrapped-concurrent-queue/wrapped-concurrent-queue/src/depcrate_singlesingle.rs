// Generated macro for Single (struct)
macro_rules! Depcrate_singleSingle {
() => {
// Module: crate::single
// Provides: {"Single"}
// Dependencies: {}
# [doc = " A single-element queue."] pub struct Single < T > { state : AtomicUsize , slot : UnsafeCell < MaybeUninit < T > > , }
};
}
