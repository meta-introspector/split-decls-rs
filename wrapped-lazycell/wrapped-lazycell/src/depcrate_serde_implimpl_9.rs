// Generated macro for impl_9 (impl)
macro_rules! Depcrate_serde_implimpl_9 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : Serialize > Serialize for LazyCell < T > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self . borrow () { Some (val) => serializer . serialize_some (val) , None => serializer . serialize_none () } } }
};
}
