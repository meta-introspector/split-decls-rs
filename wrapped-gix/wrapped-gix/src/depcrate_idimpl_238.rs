// Generated macro for impl_238 (impl)
macro_rules! Depcrate_idimpl_238 {
() => {
// Module: crate::id
// Provides: {"impl_238"}
// Dependencies: {}
impl < 'repo > Id < 'repo > { pub (crate) fn from_id (id : impl Into < ObjectId > , repo : & 'repo crate :: Repository) -> Self { Id { inner : id . into () , repo } } # [doc = " Turn this instance into its bare [`ObjectId`]."] pub fn detach (self) -> ObjectId { self . inner } }
};
}
