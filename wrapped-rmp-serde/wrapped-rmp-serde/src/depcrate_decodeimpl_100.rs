// Generated macro for impl_100 (impl)
macro_rules! Depcrate_decodeimpl_100 {
() => {
// Module: crate::decode
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > de :: SeqAccess < 'de > for SeqAccess < 'a , R , C > { type Error = Error ; # [inline] fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > { if self . left > 0 { self . left -= 1 ; Ok (Some (seed . deserialize (& mut * self . de) ?)) } else { Ok (None) } } # [inline (always)] fn size_hint (& self) -> Option < usize > { self . left . try_into () . ok () } }
};
}
