// Generated macro for impl_57 (impl)
macro_rules! Depcrate_deimpl_57 {
() => {
// Module: crate::de
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'de , T > de :: VariantAccess < 'de > for VariantAccessMap < T > where T : de :: MapAccess < 'de , Error = Error > + MakeError , { type Error = Error ; fn unit_variant (mut self) -> Result < () > { match self . map . next_value () { Ok (()) => Ok (()) , Err (e) => Err (e) , } } fn newtype_variant_seed < S > (mut self , seed : S) -> Result < S :: Value > where S : de :: DeserializeSeed < 'de > , { self . map . next_value_seed (seed) } fn tuple_variant < V > (mut self , _len : usize , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { let seed = StructVariantSeed { visitor } ; self . map . next_value_seed (seed) } fn struct_variant < V > (mut self , _fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { let seed = StructVariantSeed { visitor } ; self . map . next_value_seed (seed) } }
};
}
