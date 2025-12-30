// Generated macro for impl_47 (impl)
macro_rules! Depcrate_deimpl_47 {
() => {
// Module: crate::de
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de , T > de :: EnumAccess < 'de > for VariantAccess < T > where T : de :: SeqAccess < 'de , Error = Error > + MakeError , { type Error = Error ; type Variant = VariantAccess < T > ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , VariantAccess < T >) > where V : de :: DeserializeSeed < 'de > , { let variant = match self . seq . next_element_seed (seed) { Ok (Some (variant)) => variant , Ok (None) => return Err (self . seq . error (ErrorCode :: ArrayTooShort)) , Err (e) => return Err (e) , } ; Ok ((variant , self)) } }
};
}
