// Generated macro for impl_75 (impl)
macro_rules! Depcrate_configimpl_75 {
() => {
// Module: crate::config
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for ItemGeneric { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct ItemGenericVisitor ; impl de :: Visitor < '_ > for ItemGenericVisitor { type Value = ItemGeneric ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("item identifier") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { ItemGeneric :: from_str (value) . map_err (de :: Error :: custom) } } deserializer . deserialize_str (ItemGenericVisitor) } }
};
}
