// Generated macro for impl_64 (impl)
macro_rules! Depcrate_deimpl_64 {
() => {
// Module: crate::de
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'de , 'a , R : Read < 'de > + 'a > de :: EnumAccess < 'de > for UnitVariantAccess < 'a , R > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) > where V : de :: DeserializeSeed < 'de > , { let variant = tri ! (seed . deserialize (& mut * self . de)) ; Ok ((variant , self)) } }
};
}
