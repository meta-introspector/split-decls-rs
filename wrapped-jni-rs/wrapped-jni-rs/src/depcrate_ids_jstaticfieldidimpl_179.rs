// Generated macro for impl_179 (impl)
macro_rules! Depcrate_ids_jstaticfieldidimpl_179 {
() => {
// Module: crate::ids::jstaticfieldid
// Provides: {"impl_179"}
// Dependencies: {}
impl JStaticFieldID { # [doc = " Creates a [`JStaticFieldID`] that wraps the given `raw` [`jfieldID`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Expects a valid, non-`null` ID"] pub const unsafe fn from_raw (raw : jfieldID) -> Self { Self { internal : raw } } # [doc = " Unwrap to the internal jni type."] pub const fn into_raw (self) -> jfieldID { self . internal } }
};
}
