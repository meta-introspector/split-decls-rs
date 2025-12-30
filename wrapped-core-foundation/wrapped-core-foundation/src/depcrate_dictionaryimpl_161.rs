// Generated macro for impl_161 (impl)
macro_rules! Depcrate_dictionaryimpl_161 {
() => {
// Module: crate::dictionary
// Provides: {"impl_161"}
// Dependencies: {}
impl < K , V > Drop for CFMutableDictionary < K , V > { fn drop (& mut self) { unsafe { CFRelease (self . as_CFTypeRef ()) } } }
};
}
