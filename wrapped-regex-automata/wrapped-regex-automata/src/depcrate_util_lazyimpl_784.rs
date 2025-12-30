// Generated macro for impl_784 (impl)
macro_rules! Depcrate_util_lazyimpl_784 {
() => {
// Module: crate::util::lazy
// Provides: {"impl_784"}
// Dependencies: {}
impl < T , F : Fn () -> T > Lazy < T , F > { # [doc = " Return a reference to the lazily initialized value."] # [doc = ""] # [doc = " This routine may block if another thread is initializing a `T`."] # [doc = ""] # [doc = " Note that given a `x` which has type `Lazy`, this must be called via"] # [doc = " `Lazy::get(x)` and not `x.get()`. This routine is defined this way"] # [doc = " because `Lazy` impls `Deref` with a target of `T`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if the `create` function inside this lazy value panics."] # [doc = " If the panic occurred in another thread, then this routine _may_ also"] # [doc = " panic (but is not guaranteed to do so)."] pub fn get (this : & Lazy < T , F >) -> & T { this . 0 . get () } }
};
}
