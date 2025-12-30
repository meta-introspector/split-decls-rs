// Generated macro for MapAccess (trait)
macro_rules! Depcrate_deMapAccess {
() => {
// Module: crate::de
// Provides: {"MapAccess"}
// Dependencies: {}
pub trait MapAccess < 'de > { fn erased_next_key (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > ; fn erased_next_value (& mut self , seed : & mut dyn DeserializeSeed < 'de >) -> Result < Out , Error > ; fn erased_next_entry (& mut self , key : & mut dyn DeserializeSeed < 'de > , value : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < (Out , Out) > , Error > ; fn erased_size_hint (& self) -> Option < usize > ; }
};
}
