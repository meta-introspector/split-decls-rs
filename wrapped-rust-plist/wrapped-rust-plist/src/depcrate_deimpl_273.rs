// Generated macro for impl_273 (impl)
macro_rules! Depcrate_deimpl_273 {
() => {
// Module: crate::de
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'de , 'event , I > de :: EnumAccess < 'de > for & mut Deserializer < 'event , I > where I : IntoIterator < Item = Result < Event < 'event > , Error > > , { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) , Error > where V : de :: DeserializeSeed < 'de > , { Ok ((seed . deserialize (& mut * self) ? , self)) } }
};
}
