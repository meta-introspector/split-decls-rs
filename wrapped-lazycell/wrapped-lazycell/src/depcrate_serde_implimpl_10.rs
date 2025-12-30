// Generated macro for impl_10 (impl)
macro_rules! Depcrate_serde_implimpl_10 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Serialize > Serialize for AtomicLazyCell < T > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self . borrow () { Some (val) => serializer . serialize_some (val) , None => serializer . serialize_none () } } }
};
}
