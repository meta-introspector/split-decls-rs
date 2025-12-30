// Generated macro for impl_389 (impl)
macro_rules! Depcrate_value_deimpl_389 {
() => {
// Module: crate::value::de
// Provides: {"impl_389"}
// Dependencies: {}
impl < 'de > de :: VariantAccess < 'de > for UnitOnly { type Error = Error ; fn unit_variant (self) -> Result < () , Error > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value , Error > where T : de :: DeserializeSeed < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) } }
};
}
