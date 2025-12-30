// Generated macro for impl_33 (impl)
macro_rules! Depcrate_deimpl_33 {
() => {
// Module: crate::de
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'de , 'a , R > de :: SeqAccess < 'de > for SeqAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : de :: DeserializeSeed < 'de > , { if * self . len == 0 { return Ok (None) ; } * self . len -= 1 ; let value = seed . deserialize (& mut * self . de) ? ; Ok (Some (value)) } fn size_hint (& self) -> Option < usize > { Some (* self . len) } }
};
}
