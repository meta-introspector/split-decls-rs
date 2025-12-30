// Generated macro for impl_45 (impl)
macro_rules! Depcrate_deimpl_45 {
() => {
// Module: crate::de
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'de , 'a , R > de :: VariantAccess < 'de > for UnitVariantAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; fn unit_variant (self) -> Result < () > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value > where T : de :: DeserializeSeed < 'de > , { Err (de :: Error :: invalid_type (de :: Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (de :: Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (de :: Unexpected :: UnitVariant , & "struct variant" ,)) } }
};
}
