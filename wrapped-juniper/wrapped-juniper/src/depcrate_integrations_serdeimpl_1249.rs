// Generated macro for impl_1249 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1249 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1249"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for DefaultScalarValue { fn deserialize < D : Deserializer < 'de > > (de : D) -> Result < Self , D :: Error > { struct Visitor ; impl de :: Visitor < '_ > for Visitor { type Value = DefaultScalarValue ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("a valid input value") } fn visit_bool < E : de :: Error > (self , b : bool) -> Result < Self :: Value , E > { Ok (DefaultScalarValue :: Boolean (b)) } fn visit_i64 < E : de :: Error > (self , n : i64) -> Result < Self :: Value , E > { if n >= i64 :: from (i32 :: MIN) && n <= i64 :: from (i32 :: MAX) { Ok (DefaultScalarValue :: Int (n . try_into () . unwrap ())) } else { Ok (DefaultScalarValue :: Float (n as f64)) } } fn visit_u64 < E : de :: Error > (self , n : u64) -> Result < Self :: Value , E > { if n <= u64 :: try_from (i32 :: MAX) . unwrap () { self . visit_i64 (n . try_into () . unwrap ()) } else { Ok (DefaultScalarValue :: Float (n as f64)) } } fn visit_f64 < E : de :: Error > (self , f : f64) -> Result < Self :: Value , E > { Ok (DefaultScalarValue :: Float (f)) } fn visit_str < E : de :: Error > (self , s : & str) -> Result < Self :: Value , E > { self . visit_string (s . into ()) } fn visit_string < E : de :: Error > (self , s : String) -> Result < Self :: Value , E > { Ok (DefaultScalarValue :: String (s)) } } de . deserialize_any (Visitor) } }
};
}
