// Generated macro for impl_103 (impl)
macro_rules! Depcrate_decodeimpl_103 {
() => {
// Module: crate::decode
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > de :: MapAccess < 'de > for MapAccess < 'a , R , C > { type Error = Error ; # [inline] fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : DeserializeSeed < 'de > { if self . left > 0 { self . left -= 1 ; seed . deserialize (& mut * self . de) . map (Some) } else { Ok (None) } } # [inline] fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : DeserializeSeed < 'de > { seed . deserialize (& mut * self . de) } # [inline (always)] fn size_hint (& self) -> Option < usize > { self . left . try_into () . ok () } }
};
}
