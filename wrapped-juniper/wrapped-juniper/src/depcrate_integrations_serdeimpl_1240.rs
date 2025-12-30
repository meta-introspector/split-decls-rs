// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1240 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1240"}
// Dependencies: {}
impl < T : Serialize > Serialize for ExecutionError < T > { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { let mut map = ser . serialize_map (Some (4)) ? ; map . serialize_key ("message") ? ; map . serialize_value (self . error () . message ()) ? ; let locations = vec ! [self . location ()] ; map . serialize_key ("locations") ? ; map . serialize_value (& locations) ? ; map . serialize_key ("path") ? ; map . serialize_value (self . path ()) ? ; if ! self . error () . extensions () . is_null () { map . serialize_key ("extensions") ? ; map . serialize_value (self . error () . extensions ()) ? ; } map . end () } }
};
}
