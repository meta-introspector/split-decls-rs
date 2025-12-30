// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1246 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1246"}
// Dependencies: {}
impl Serialize for Spanning < ParseError > { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { let mut map = ser . serialize_map (Some (2)) ? ; let msg = self . item . to_string () ; map . serialize_key ("message") ? ; map . serialize_value (& msg) ? ; let mut loc = IndexMap :: new () ; loc . insert ("line" . to_owned () , self . start () . line () + 1) ; loc . insert ("column" . to_owned () , self . start () . column () + 1) ; let locations = vec ! [loc] ; map . serialize_key ("locations") ? ; map . serialize_value (& locations) ? ; map . end () } }
};
}
