// Generated macro for impl_755 (impl)
macro_rules! Depcrate_rawimpl_755 {
() => {
// Module: crate::raw
// Provides: {"impl_755"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Box < RawValue > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct BoxedVisitor ; impl < 'de > Visitor < 'de > for BoxedVisitor { type Value = Box < RawValue > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "any valid JSON value") } fn visit_map < V > (self , mut visitor : V) -> Result < Self :: Value , V :: Error > where V : MapAccess < 'de > , { let value = tri ! (visitor . next_key ::< RawKey > ()) ; if value . is_none () { return Err (de :: Error :: invalid_type (Unexpected :: Map , & self)) ; } visitor . next_value_seed (BoxedFromString) } } deserializer . deserialize_newtype_struct (TOKEN , BoxedVisitor) } }
};
}
