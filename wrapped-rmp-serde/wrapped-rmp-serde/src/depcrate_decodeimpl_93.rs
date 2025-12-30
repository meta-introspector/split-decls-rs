// Generated macro for impl_93 (impl)
macro_rules! Depcrate_decodeimpl_93 {
() => {
// Module: crate::decode
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > de :: SeqAccess < 'de > for ExtDeserializer < 'a , R , C > { type Error = Error ; # [inline] fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : DeserializeSeed < 'de > , { match self . state { ExtDeserializerState :: New | ExtDeserializerState :: ReadTag => Ok (Some (seed . deserialize (self) ?)) , ExtDeserializerState :: ReadBinary => Ok (None) , } } }
};
}
