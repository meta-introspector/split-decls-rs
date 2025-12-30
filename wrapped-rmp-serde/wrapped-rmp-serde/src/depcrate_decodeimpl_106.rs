// Generated macro for impl_106 (impl)
macro_rules! Depcrate_decodeimpl_106 {
() => {
// Module: crate::decode
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'de , R : ReadSlice < 'de > , C : SerializerConfig > de :: EnumAccess < 'de > for UnitVariantAccess < '_ , R , C > { type Error = Error ; type Variant = Self ; # [inline] fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) , Error > where V : de :: DeserializeSeed < 'de > , { let variant = seed . deserialize (& mut * self . de) ? ; Ok ((variant , self)) } }
};
}
