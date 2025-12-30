// Generated macro for impl_18 (impl)
macro_rules! Depcrate_dev_macimpl_18 {
() => {
// Module: crate::dev::mac
// Provides: {"impl_18"}
// Dependencies: {}
impl < T : OutputSizeUser > CtOutput < T > { # [doc = " Create a new [`CtOutput`] value."] # [inline (always)] pub fn new (bytes : Output < T >) -> Self { Self { bytes } } # [doc = " Get reference to the inner [`Output`] array this type wraps."] # [inline (always)] pub fn as_bytes (& self) -> & Output < T > { & self . bytes } # [doc = " Get the inner [`Output`] array this type wraps."] # [inline (always)] pub fn into_bytes (& self) -> Output < T > { self . bytes . clone () } }
};
}
