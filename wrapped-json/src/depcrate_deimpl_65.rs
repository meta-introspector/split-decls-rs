// Generated macro for impl_65 (impl)
macro_rules! Depcrate_deimpl_65 {
() => {
// Module: crate::de
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'de , 'a , R : Read < 'de > + 'a > de :: VariantAccess < 'de > for UnitVariantAccess < 'a , R > { type Error = Error ; fn unit_variant (self) -> Result < () > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value > where T : de :: DeserializeSeed < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) } }
};
}
