// Generated macro for impl_283 (impl)
macro_rules! Depcrate_value_rawimpl_283 {
() => {
// Module: crate::value::raw
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Box < RawValue > { fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct BoxedVisitor ; impl < 'de > de :: Visitor < 'de > for BoxedVisitor { type Value = Box < RawValue > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "any valid RON-value-string") } fn visit_str < E : de :: Error > (self , ron : & str) -> Result < Self :: Value , E > { match Options :: default () . from_str :: < de :: IgnoredAny > (ron) { Ok (_) => Ok (RawValue :: from_boxed_str (ron . to_owned () . into_boxed_str ())) , Err (err) => Err (de :: Error :: custom (format ! ("invalid RON value at {}" , err))) , } } fn visit_string < E : de :: Error > (self , ron : String) -> Result < Self :: Value , E > { match Options :: default () . from_str :: < de :: IgnoredAny > (& ron) { Ok (_) => Ok (RawValue :: from_boxed_str (ron . into_boxed_str ())) , Err (err) => Err (de :: Error :: custom (format ! ("invalid RON value at {}" , err))) , } } fn visit_newtype_struct < D : de :: Deserializer < 'de > > (self , deserializer : D ,) -> Result < Self :: Value , D :: Error > { deserializer . deserialize_string (self) } } deserializer . deserialize_newtype_struct (RAW_VALUE_TOKEN , BoxedVisitor) } }
};
}
