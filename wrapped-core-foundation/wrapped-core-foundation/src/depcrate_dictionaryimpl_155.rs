// Generated macro for impl_155 (impl)
macro_rules! Depcrate_dictionaryimpl_155 {
() => {
// Module: crate::dictionary
// Provides: {"impl_155"}
// Dependencies: {}
impl < K , V > Drop for CFDictionary < K , V > { fn drop (& mut self) { unsafe { CFRelease (self . as_CFTypeRef ()) } } }
};
}
