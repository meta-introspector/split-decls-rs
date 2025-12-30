// Generated macro for impl_17 (impl)
macro_rules! Depcrate_unsafe_wrapperimpl_17 {
() => {
// Module: crate::unsafe_wrapper
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > Unsafe < T > { # [doc = " Create a new `Unsafe`, asserting that all invariants are upheld."] # [inline (always)] pub (crate) const unsafe fn new (value : T) -> Self { Self (value) } # [doc = " Get a reference to the inner value."] # [inline (always)] pub (crate) const fn get (& self) -> & T { & self . 0 } }
};
}
