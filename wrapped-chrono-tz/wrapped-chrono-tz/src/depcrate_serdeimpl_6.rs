// Generated macro for impl_6 (impl)
macro_rules! Depcrate_serdeimpl_6 {
() => {
// Module: crate::serde
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Tz { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = Tz ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "an IANA timezone string") } fn visit_str < E : de :: Error > (self , value : & str) -> Result < Tz , E > { value . parse :: < Tz > () . map_err (| _ | E :: custom (SerdeError (value))) } } deserializer . deserialize_str (Visitor) } }
};
}
