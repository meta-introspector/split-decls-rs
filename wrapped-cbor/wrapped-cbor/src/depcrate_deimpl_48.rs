// Generated macro for impl_48 (impl)
macro_rules! Depcrate_deimpl_48 {
() => {
// Module: crate::de
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'de , T > de :: VariantAccess < 'de > for VariantAccess < T > where T : de :: SeqAccess < 'de , Error = Error > + MakeError , { type Error = Error ; fn unit_variant (mut self) -> Result < () > { match self . seq . next_element () { Ok (Some (())) => Ok (()) , Ok (None) => Err (self . seq . error (ErrorCode :: ArrayTooLong)) , Err (e) => Err (e) , } } fn newtype_variant_seed < S > (mut self , seed : S) -> Result < S :: Value > where S : de :: DeserializeSeed < 'de > , { match self . seq . next_element_seed (seed) { Ok (Some (variant)) => Ok (variant) , Ok (None) => Err (self . seq . error (ErrorCode :: ArrayTooShort)) , Err (e) => Err (e) , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { visitor . visit_seq (self . seq) } fn struct_variant < V > (mut self , _fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { let seed = StructVariantSeed { visitor } ; match self . seq . next_element_seed (seed) { Ok (Some (variant)) => Ok (variant) , Ok (None) => Err (self . seq . error (ErrorCode :: ArrayTooShort)) , Err (e) => Err (e) , } } }
};
}
