// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1244 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1244"}
// Dependencies: {}
impl Serialize for RuleError { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { let mut map = ser . serialize_map (Some (2)) ? ; map . serialize_key ("message") ? ; map . serialize_value (self . message ()) ? ; map . serialize_key ("locations") ? ; map . serialize_value (self . locations ()) ? ; map . end () } }
};
}
