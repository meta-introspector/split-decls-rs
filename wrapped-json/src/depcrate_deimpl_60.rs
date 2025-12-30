// Generated macro for impl_60 (impl)
macro_rules! Depcrate_deimpl_60 {
() => {
// Module: crate::de
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'de , 'a , R : Read < 'de > + 'a > de :: EnumAccess < 'de > for VariantAccess < 'a , R > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) > where V : de :: DeserializeSeed < 'de > , { let val = tri ! (seed . deserialize (& mut * self . de)) ; tri ! (self . de . parse_object_colon ()) ; Ok ((val , self)) } }
};
}
