// Generated macro for impl_326 (impl)
macro_rules! Depcrate_value_deimpl_326 {
() => {
// Module: crate::value::de
// Provides: {"impl_326"}
// Dependencies: {}
impl < 'de > de :: EnumAccess < 'de > for BorrowedCowStrDeserializer < 'de > { type Error = Error ; type Variant = UnitOnly ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Error > where T : de :: DeserializeSeed < 'de > , { let value = tri ! (seed . deserialize (self)) ; Ok ((value , UnitOnly)) } }
};
}
