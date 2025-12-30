// Generated macro for impl_101 (impl)
macro_rules! Depcrate_collections_vecimpl_101 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'bump , T : 'bump > ops :: Deref for Vec < 'bump , T > { type Target = [T] ; fn deref (& self) -> & [T] { unsafe { let p = self . buf . ptr () ; slice :: from_raw_parts (p , self . len) } } }
};
}
