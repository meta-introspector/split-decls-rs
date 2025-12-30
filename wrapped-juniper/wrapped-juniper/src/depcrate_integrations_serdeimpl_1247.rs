// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1247 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1247"}
// Dependencies: {}
impl < T : Serialize > Serialize for Object < T > { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { let mut map = ser . serialize_map (Some (self . field_count ())) ? ; for (f , v) in self . iter () { map . serialize_key (f) ? ; map . serialize_value (v) ? ; } map . end () } }
};
}
