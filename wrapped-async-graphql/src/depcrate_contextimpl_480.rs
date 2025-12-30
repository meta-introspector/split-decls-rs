// Generated macro for impl_480 (impl)
macro_rules! Depcrate_contextimpl_480 {
() => {
// Module: crate::context
// Provides: {"impl_480"}
// Dependencies: {}
impl serde :: Serialize for QueryPathNode < '_ > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut seq = serializer . serialize_seq (None) ? ; self . try_for_each (| segment | seq . serialize_element (segment)) ? ; seq . end () } }
};
}
