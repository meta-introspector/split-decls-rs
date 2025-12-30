// Generated macro for impl_417 (impl)
macro_rules! Depcrate_serializeimpl_417 {
() => {
// Module: crate::serialize
// Provides: {"impl_417"}
// Dependencies: {}
impl Deref for Data < '_ > { type Target = [u8] ; fn deref (& self) -> & [u8] { let (ptr , sz) = match self { Data :: Owned (OwnedData { ptr , sz }) => (ptr . as_ptr () , * sz) , Data :: Shared (SharedData { ptr , sz , .. }) => (ptr . as_ptr () , * sz) , } ; unsafe { std :: slice :: from_raw_parts (ptr , sz) } } }
};
}
