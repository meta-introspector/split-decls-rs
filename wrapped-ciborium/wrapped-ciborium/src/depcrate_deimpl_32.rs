// Generated macro for impl_32 (impl)
macro_rules! Depcrate_deimpl_32 {
() => {
// Module: crate::de
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de , R : Read > de :: SeqAccess < 'de > for BytesAccess < R > where R :: Error : core :: fmt :: Debug , { type Error = Error < R :: Error > ; # [inline] fn next_element_seed < U : de :: DeserializeSeed < 'de > > (& mut self , seed : U ,) -> Result < Option < U :: Value > , Self :: Error > { use de :: IntoDeserializer ; if self . 0 < self . 1 . len () { let byte = self . 1 [self . 0] ; self . 0 += 1 ; seed . deserialize (byte . into_deserializer ()) . map (Some) } else { Ok (None) } } # [inline] fn size_hint (& self) -> Option < usize > { Some (self . 1 . len () - self . 0) } }
};
}
