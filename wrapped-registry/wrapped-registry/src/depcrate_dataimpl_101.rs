// Generated macro for impl_101 (impl)
macro_rules! Depcrate_dataimpl_101 {
() => {
// Module: crate::data
// Provides: {"impl_101"}
// Dependencies: {}
impl Deref for Data { type Target = [u8] ; fn deref (& self) -> & [u8] { if self . ptr . is_null () { & [] } else { unsafe { core :: slice :: from_raw_parts (self . ptr , self . len) } } } }
};
}
