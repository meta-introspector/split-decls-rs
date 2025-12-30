// Generated macro for impl_89 (impl)
macro_rules! Depcrate_tagimpl_89 {
() => {
// Module: crate::tag
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'de , D : de :: Deserializer < 'de > > de :: EnumAccess < 'de > for TagAccess < D > { type Error = D :: Error ; type Variant = Self ; # [inline] fn variant_seed < V : de :: DeserializeSeed < 'de > > (mut self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > { let variant = seed . deserialize (& mut self) ? ; Ok ((variant , self)) } }
};
}
