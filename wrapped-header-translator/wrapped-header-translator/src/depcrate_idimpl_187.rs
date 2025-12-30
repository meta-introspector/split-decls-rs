// Generated macro for impl_187 (impl)
macro_rules! Depcrate_idimpl_187 {
() => {
// Module: crate::id
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for Location { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct LocationVisitor ; impl de :: Visitor < '_ > for LocationVisitor { type Value = Location ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("location") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { Location :: from_str (value) . map_err (de :: Error :: custom) } } deserializer . deserialize_str (LocationVisitor) } }
};
}
