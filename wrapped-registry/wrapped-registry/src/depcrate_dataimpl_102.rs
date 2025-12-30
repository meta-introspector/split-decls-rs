// Generated macro for impl_102 (impl)
macro_rules! Depcrate_dataimpl_102 {
() => {
// Module: crate::data
// Provides: {"impl_102"}
// Dependencies: {}
impl core :: ops :: DerefMut for Data { fn deref_mut (& mut self) -> & mut [u8] { if self . ptr . is_null () { & mut [] } else { unsafe { core :: slice :: from_raw_parts_mut (self . ptr , self . len) } } } }
};
}
