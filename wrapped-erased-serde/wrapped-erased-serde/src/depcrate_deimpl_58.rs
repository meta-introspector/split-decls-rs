// Generated macro for impl_58 (impl)
macro_rules! Depcrate_deimpl_58 {
() => {
// Module: crate::de
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'de > serde :: de :: SeqAccess < 'de > for & mut (dyn SeqAccess < 'de > + '_) { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : serde :: de :: DeserializeSeed < 'de > , { let mut seed = erase :: DeserializeSeed :: new (seed) ; unsafe { (* * self) . erased_next_element (& mut seed) . map (| opt | opt . unsafe_map (Out :: take)) } } fn size_hint (& self) -> Option < usize > { (* * self) . erased_size_hint () } }
};
}
