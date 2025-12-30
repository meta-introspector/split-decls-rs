// Generated macro for impl_45 (impl)
macro_rules! Depcrate_deimpl_45 {
() => {
// Module: crate::de
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'de , T > MapAccess < 'de > for erase :: MapAccess < T > where T : serde :: de :: MapAccess < 'de > , { fn erased_next_key (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > { self . as_mut () . next_key_seed (seed) . map_err (erase) } fn erased_next_value (& mut self , seed : & mut dyn DeserializeSeed < 'de >) -> Result < Out , Error > { self . as_mut () . next_value_seed (seed) . map_err (erase) } fn erased_next_entry (& mut self , kseed : & mut dyn DeserializeSeed < 'de > , vseed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < (Out , Out) > , Error > { self . as_mut () . next_entry_seed (kseed , vseed) . map_err (erase) } fn erased_size_hint (& self) -> Option < usize > { self . as_ref () . size_hint () } }
};
}
