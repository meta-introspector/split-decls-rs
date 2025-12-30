// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'cs > CriticalSection < 'cs > { # [doc = " Creates a critical section token."] # [doc = ""] # [doc = " This method is meant to be used to create safe abstractions rather than being directly used"] # [doc = " in applications."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This must only be called when the current thread is in a critical section. The caller must"] # [doc = " ensure that the returned instance will not live beyond the end of the critical section."] # [doc = ""] # [doc = " The caller must use adequate fences to prevent the compiler from moving the"] # [doc = " instructions inside the critical section to the outside of it. Sequentially consistent fences are"] # [doc = " suggested immediately after entry and immediately before exit from the critical section."] # [doc = ""] # [doc = " Note that the lifetime `'cs` of the returned instance is unconstrained. User code must not"] # [doc = " be able to influence the lifetime picked for this type, since that might cause it to be"] # [doc = " inferred to `'static`."] # [inline (always)] pub unsafe fn new () -> Self { CriticalSection { _private : PhantomData , _not_send_sync : PhantomData , } } }
};
}
