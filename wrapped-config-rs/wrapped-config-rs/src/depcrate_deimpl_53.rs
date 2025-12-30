// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deimpl_53 {
() => {
// Module: crate::de
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de > de :: EnumAccess < 'de > for EnumAccess { type Error = ConfigError ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) > where V : de :: DeserializeSeed < 'de > , { let value = { let deserializer = match self . value . kind { ValueKind :: String (ref s) => self . variant_deserializer (s) , ValueKind :: Table (ref t) => self . table_deserializer (t) , _ => Err (self . structural_error ()) , } ? ; seed . deserialize (deserializer) ? } ; Ok ((value , self)) } }
};
}
