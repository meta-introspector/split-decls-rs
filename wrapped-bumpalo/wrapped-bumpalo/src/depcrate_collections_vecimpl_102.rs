// Generated macro for impl_102 (impl)
macro_rules! Depcrate_collections_vecimpl_102 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'bump , T : 'bump > ops :: DerefMut for Vec < 'bump , T > { fn deref_mut (& mut self) -> & mut [T] { unsafe { let ptr = self . buf . ptr () ; slice :: from_raw_parts_mut (ptr , self . len) } } }
};
}
