// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < F : EventListenerFuture > FutureWrapper < F > { # [doc = " Create a new `FutureWrapper` from the provided future."] # [inline] pub fn new (inner : F) -> Self { Self { inner } } # [doc = " Consume the `FutureWrapper`, returning the inner future."] # [inline] pub fn into_inner (self) -> F { self . inner } }
};
}
