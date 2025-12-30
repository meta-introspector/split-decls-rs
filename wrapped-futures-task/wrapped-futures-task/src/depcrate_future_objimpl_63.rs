// Generated macro for impl_63 (impl)
macro_rules! Depcrate_future_objimpl_63 {
() => {
// Module: crate::future_obj
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , T > FutureObj < 'a , T > { # [doc = " Create a `FutureObj` from a custom trait object representation."] # [inline] pub fn new < F : UnsafeFutureObj < 'a , T > + Send > (f : F) -> Self { Self (LocalFutureObj :: new (f)) } }
};
}
