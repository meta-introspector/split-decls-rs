// Generated macro for impl_149 (impl)
macro_rules! Depcrate_deimpl_149 {
() => {
// Module: crate::de
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'de , E > Deserializer < 'de , SliceReader < 'de > , E > where E : EntityResolver , { # [doc = " Create a new deserializer that will borrow data from the specified string"] # [doc = " and use the specified entity resolver."] pub fn from_str_with_resolver (source : & 'de str , entity_resolver : E) -> Self { Self :: borrowing_with_resolver (NsReader :: from_str (source) , entity_resolver) } # [doc = " Create a new deserializer that will borrow data from the specified preconfigured"] # [doc = " reader and use the specified entity resolver."] # [doc = ""] # [doc = " Note, that config option [`Config::expand_empty_elements`] will be set to `true`."] # [doc = ""] # [doc = " [`Config::expand_empty_elements`]: crate::reader::Config::expand_empty_elements"] pub fn borrowing_with_resolver (mut reader : NsReader < & 'de [u8] > , entity_resolver : E) -> Self { let config = reader . config_mut () ; config . expand_empty_elements = true ; Self :: new (SliceReader { reader } , entity_resolver) } }
};
}
