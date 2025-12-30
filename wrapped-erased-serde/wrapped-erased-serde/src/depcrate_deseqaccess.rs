// Generated macro for SeqAccess (trait)
macro_rules! Depcrate_deSeqAccess {
() => {
// Module: crate::de
// Provides: {"SeqAccess"}
// Dependencies: {}
pub trait SeqAccess < 'de > { fn erased_next_element (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > ; fn erased_size_hint (& self) -> Option < usize > ; }
};
}
