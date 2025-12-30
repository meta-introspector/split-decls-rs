// Generated macro for impl_61 (impl)
macro_rules! Depcrate_deserializerimpl_61 {
() => {
// Module: crate::deserializer
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , 'de : 'a , T : DeRecord < 'de > > SeqAccess < 'de > for & 'a mut DeRecordWrap < T > { type Error = DeserializeError ; fn next_element_seed < U : DeserializeSeed < 'de > > (& mut self , seed : U ,) -> Result < Option < U :: Value > , Self :: Error > { if self . peek_field () . is_none () { Ok (None) } else { seed . deserialize (& mut * * self) . map (Some) } } }
};
}
