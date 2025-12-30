// Generated macro for impl_108 (impl)
macro_rules! Depcrate_jsontimpl_108 {
() => {
// Module: crate::jsont
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a > serde :: Serialize for Data < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("Data" , 1) ? ; match * self { Data :: Text { ref text } => state . serialize_field ("text" , text) ? , Data :: Bytes { bytes } => { state . serialize_field ("bytes" , & base64_standard (bytes)) ? ; } } state . end () } }
};
}
