// Generated macro for impl_154 (impl)
macro_rules! Depcrate_value_deimpl_154 {
() => {
// Module: crate::value::de
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a , 'de > de :: EnumAccess < 'de > for Deserializer < & 'a (Value , Value) > { type Error = Error ; type Variant = Deserializer < & 'a Value > ; # [inline] fn variant_seed < V : de :: DeserializeSeed < 'de > > (self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > { let k = seed . deserialize (Deserializer (& self . 0 . 0)) ? ; Ok ((k , Deserializer (& self . 0 . 1))) } }
};
}
