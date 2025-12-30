// Generated macro for impl_818 (impl)
macro_rules! Depcrate_rawimpl_818 {
() => {
// Module: crate::raw
// Provides: {"impl_818"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for RawKey { fn deserialize < D > (deserializer : D) -> Result < RawKey , D :: Error > where D : Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > Visitor < 'de > for FieldVisitor { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("raw value") } fn visit_str < E > (self , s : & str) -> Result < () , E > where E : de :: Error , { if s == TOKEN { Ok (()) } else { Err (de :: Error :: custom ("unexpected raw value")) } } } tri ! (deserializer . deserialize_identifier (FieldVisitor)) ; Ok (RawKey) } }
};
}
