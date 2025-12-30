// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1245 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1245"}
// Dependencies: {}
impl Serialize for SourcePosition { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { let mut map = ser . serialize_map (Some (2)) ? ; let line = self . line () + 1 ; map . serialize_key ("line") ? ; map . serialize_value (& line) ? ; let column = self . column () + 1 ; map . serialize_key ("column") ? ; map . serialize_value (& column) ? ; map . end () } }
};
}
