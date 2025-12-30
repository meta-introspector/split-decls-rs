// Generated macro for impl_37 (impl)
macro_rules! Depcrate_atomicimpl_37 {
() => {
// Module: crate::atomic
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > Atomic < T > { # [doc = " Allocates `value` on the heap and returns a new atomic pointer pointing to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Atomic;"] # [doc = ""] # [doc = " let a = Atomic::new(1234);"] # [doc = " # unsafe { drop(a.into_owned()); } // avoid leak"] # [doc = " ```"] pub fn new (init : T) -> Self { Self :: init (init) } }
};
}
