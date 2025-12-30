// Generated macro for impl_10 (impl)
macro_rules! Depcrate_serdeimpl_10 {
() => {
// Module: crate::serde
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Serialize > Serialize for VecList < T > { fn serialize < U : Serializer > (& self , serializer : U) -> Result < U :: Ok , U :: Error > { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for value in self . iter () { seq . serialize_element (value) ? ; } seq . end () } }
};
}
