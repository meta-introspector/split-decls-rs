// Generated macro for impl_100 (impl)
macro_rules! Depcrate_deimpl_100 {
() => {
// Module: crate::de
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'de , 'a > de :: EnumAccess < 'de > for Enum < 'a , 'de > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) > where V : DeserializeSeed < 'de > , { self . de . parser . skip_ws () ? ; let value = guard_recursion ! { self . de => seed . deserialize (& mut * self . de) ? } ; Ok ((value , self)) } }
};
}
