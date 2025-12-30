// Generated macro for impl_91 (impl)
macro_rules! Depcrate_tagimpl_91 {
() => {
// Module: crate::tag
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'de , D : de :: Deserializer < 'de > > de :: SeqAccess < 'de > for TagAccess < D > { type Error = D :: Error ; # [inline] fn next_element_seed < T : de :: DeserializeSeed < 'de > > (& mut self , seed : T ,) -> Result < Option < T :: Value > , Self :: Error > { if self . state < 2 { return Ok (Some (seed . deserialize (self) ?)) ; } Ok (match self . parent . take () { Some (x) => Some (seed . deserialize (x) ?) , None => None , }) } }
};
}
