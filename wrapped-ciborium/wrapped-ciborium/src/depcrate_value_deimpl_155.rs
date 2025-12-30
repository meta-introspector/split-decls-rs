// Generated macro for impl_155 (impl)
macro_rules! Depcrate_value_deimpl_155 {
() => {
// Module: crate::value::de
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , 'de > de :: EnumAccess < 'de > for Deserializer < & 'a Value > { type Error = Error ; type Variant = Deserializer < & 'a Value > ; # [inline] fn variant_seed < V : de :: DeserializeSeed < 'de > > (self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > { let k = seed . deserialize (self) ? ; Ok ((k , Deserializer (& Value :: Null))) } }
};
}
