// Generated macro for impl_110 (impl)
macro_rules! Depcrate_decodeimpl_110 {
() => {
// Module: crate::decode
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'de , R : ReadSlice < 'de > , C : SerializerConfig > de :: EnumAccess < 'de > for VariantAccess < '_ , R , C > { type Error = Error ; type Variant = Self ; # [inline] fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) , Error > where V : de :: DeserializeSeed < 'de > , { Ok ((seed . deserialize (& mut * self . de) ? , self)) } }
};
}
