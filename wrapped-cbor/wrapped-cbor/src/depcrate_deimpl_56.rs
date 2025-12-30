// Generated macro for impl_56 (impl)
macro_rules! Depcrate_deimpl_56 {
() => {
// Module: crate::de
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'de , T > de :: EnumAccess < 'de > for VariantAccessMap < T > where T : de :: MapAccess < 'de , Error = Error > + MakeError , { type Error = Error ; type Variant = VariantAccessMap < T > ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , VariantAccessMap < T >) > where V : de :: DeserializeSeed < 'de > , { let variant = match self . map . next_key_seed (seed) { Ok (Some (variant)) => variant , Ok (None) => return Err (self . map . error (ErrorCode :: ArrayTooShort)) , Err (e) => return Err (e) , } ; Ok ((variant , self)) } }
};
}
