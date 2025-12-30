// Generated macro for impl_282 (impl)
macro_rules! Depcrate_value_rawimpl_282 {
() => {
// Module: crate::value::raw
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a RawValue { fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct ReferenceVisitor ; impl < 'de > de :: Visitor < 'de > for ReferenceVisitor { type Value = & 'de RawValue ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "any valid borrowed RON-value-string") } fn visit_borrowed_str < E : de :: Error > (self , ron : & 'de str) -> Result < Self :: Value , E > { match Options :: default () . from_str :: < de :: IgnoredAny > (ron) { Ok (_) => Ok (RawValue :: from_borrowed_str (ron)) , Err (err) => Err (de :: Error :: custom (format ! ("invalid RON value at {}" , err))) , } } fn visit_newtype_struct < D : de :: Deserializer < 'de > > (self , deserializer : D ,) -> Result < Self :: Value , D :: Error > { deserializer . deserialize_str (self) } } deserializer . deserialize_newtype_struct (RAW_VALUE_TOKEN , ReferenceVisitor) } }
};
}
