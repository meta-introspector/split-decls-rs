// Generated macro for impl_179 (impl)
macro_rules! Depcrate_setimpl_179 {
() => {
// Module: crate::set
// Provides: {"impl_179"}
// Dependencies: {}
impl < K : Eq + Hash , S : BuildHasher + Clone > IntoIterator for DashSet < K , S > { type Item = K ; type IntoIter = OwningIter < K > ; fn into_iter (self) -> Self :: IntoIter { OwningIter :: new (self . inner . into_iter ()) } }
};
}
