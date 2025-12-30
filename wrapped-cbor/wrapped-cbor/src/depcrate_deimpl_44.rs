// Generated macro for impl_44 (impl)
macro_rules! Depcrate_deimpl_44 {
() => {
// Module: crate::de
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'de , 'a , R > de :: EnumAccess < 'de > for UnitVariantAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; type Variant = UnitVariantAccess < 'a , R > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , UnitVariantAccess < 'a , R >) > where V : de :: DeserializeSeed < 'de > , { let variant = seed . deserialize (& mut * self . de) ? ; Ok ((variant , self)) } }
};
}
