// Generated macro for impl_190 (impl)
macro_rules! Depcrate_idimpl_190 {
() => {
// Module: crate::id
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for ItemIdentifier { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct ItemIdentifierVisitor ; impl de :: Visitor < '_ > for ItemIdentifierVisitor { type Value = ItemIdentifier ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("item identifier") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { ItemIdentifier :: from_str (value) . map_err (de :: Error :: custom) } } deserializer . deserialize_str (ItemIdentifierVisitor) } }
};
}
