// Generated macro for impl_62 (impl)
macro_rules! Depcrate_deimpl_62 {
() => {
// Module: crate::de
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'de > serde :: de :: VariantAccess < 'de > for Variant < 'de > { type Error = Error ; fn unit_variant (self) -> Result < () , Error > { unsafe { (self . unit_variant) (self . data) } } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Error > where T : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; unsafe { (self . visit_newtype) (self . data , & mut erased) . unsafe_map (Out :: take) } } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Error > where V : serde :: de :: Visitor < 'de > , { let mut erased = erase :: Visitor :: new (visitor) ; unsafe { (self . tuple_variant) (self . data , len , & mut erased) . unsafe_map (Out :: take) } } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : serde :: de :: Visitor < 'de > , { let mut erased = erase :: Visitor :: new (visitor) ; unsafe { (self . struct_variant) (self . data , fields , & mut erased) . unsafe_map (Out :: take) } } }
};
}
