// Generated macro for impl_170 (impl)
macro_rules! Depcrate_ids_jfieldidimpl_170 {
() => {
// Module: crate::ids::jfieldid
// Provides: {"impl_170"}
// Dependencies: {}
impl JFieldID { # [doc = " Creates a [`JFieldID`] that wraps the given `raw` [`jfieldID`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Expects a valid, non-`null` ID"] pub const unsafe fn from_raw (raw : jfieldID) -> Self { Self { internal : raw } } # [doc = " Unwrap to the internal jni type."] pub const fn into_raw (self) -> jfieldID { self . internal } }
};
}
