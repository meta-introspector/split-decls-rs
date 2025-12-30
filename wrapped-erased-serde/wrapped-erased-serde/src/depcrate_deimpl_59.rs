// Generated macro for impl_59 (impl)
macro_rules! Depcrate_deimpl_59 {
() => {
// Module: crate::de
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'de > serde :: de :: MapAccess < 'de > for & mut (dyn MapAccess < 'de > + '_) { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; unsafe { (* * self) . erased_next_key (& mut erased) . map (| opt | opt . unsafe_map (Out :: take)) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; unsafe { (* * self) . erased_next_value (& mut erased) . unsafe_map (Out :: take) } } fn size_hint (& self) -> Option < usize > { (* * self) . erased_size_hint () } }
};
}
