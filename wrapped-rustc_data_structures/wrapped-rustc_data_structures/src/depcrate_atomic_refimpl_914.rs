// Generated macro for impl_914 (impl)
macro_rules! Depcrate_atomic_refimpl_914 {
() => {
// Module: crate::atomic_ref
// Provides: {"impl_914"}
// Dependencies: {}
impl < T : 'static > AtomicRef < T > { pub const fn new (initial : & 'static T) -> AtomicRef < T > { AtomicRef (AtomicPtr :: new (initial as * const T as * mut T) , PhantomData) } pub fn swap (& self , new : & 'static T) -> & 'static T { unsafe { & * self . 0 . swap (new as * const T as * mut T , Ordering :: SeqCst) } } }
};
}
