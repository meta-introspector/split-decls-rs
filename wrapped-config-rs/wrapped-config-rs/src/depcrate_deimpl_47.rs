// Generated macro for impl_47 (impl)
macro_rules! Depcrate_deimpl_47 {
() => {
// Module: crate::de
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de > de :: SeqAccess < 'de > for SeqAccess { type Error = ConfigError ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : de :: DeserializeSeed < 'de > , { match self . elements . next () { Some ((idx , value)) => seed . deserialize (value) . map (Some) . map_err (| e | e . prepend_index (idx)) , None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { match self . elements . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
};
}
