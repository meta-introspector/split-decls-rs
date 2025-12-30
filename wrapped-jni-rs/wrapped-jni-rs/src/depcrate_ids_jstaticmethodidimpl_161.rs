// Generated macro for impl_161 (impl)
macro_rules! Depcrate_ids_jstaticmethodidimpl_161 {
() => {
// Module: crate::ids::jstaticmethodid
// Provides: {"impl_161"}
// Dependencies: {}
impl JStaticMethodID { # [doc = " Creates a [`JStaticMethodID`] that wraps the given `raw` [`jmethodID`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Expects a valid, non-`null` ID"] pub const unsafe fn from_raw (raw : jmethodID) -> Self { Self { internal : raw } } # [doc = " Unwrap to the internal jni type."] pub const fn into_raw (self) -> jmethodID { self . internal } }
};
}
