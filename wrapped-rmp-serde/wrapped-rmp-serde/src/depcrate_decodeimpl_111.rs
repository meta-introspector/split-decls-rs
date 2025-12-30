// Generated macro for impl_111 (impl)
macro_rules! Depcrate_decodeimpl_111 {
() => {
// Module: crate::decode
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'de , R : ReadSlice < 'de > , C : SerializerConfig > de :: VariantAccess < 'de > for VariantAccess < '_ , R , C > { type Error = Error ; # [inline] fn unit_variant (self) -> Result < () , Error > { decode :: read_nil (& mut self . de . rd) ? ; Ok (()) } # [inline] fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > { seed . deserialize (self . de) } # [inline] fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > { de :: Deserializer :: deserialize_tuple (self . de , len , visitor) } # [inline] fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > { de :: Deserializer :: deserialize_tuple (self . de , fields . len () , visitor) } }
};
}
