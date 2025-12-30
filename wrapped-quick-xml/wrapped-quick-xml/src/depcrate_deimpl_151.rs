// Generated macro for impl_151 (impl)
macro_rules! Depcrate_deimpl_151 {
() => {
// Module: crate::de
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'de , R , E > Deserializer < 'de , IoReader < R > , E > where R : BufRead , E : EntityResolver , { # [doc = " Create a new deserializer that will copy data from the specified reader"] # [doc = " into internal buffer and use the specified entity resolver."] # [doc = ""] # [doc = " If you already have a string use [`Self::from_str`] instead, because it"] # [doc = " will borrow instead of copy. If you have `&[u8]` which is known to represent"] # [doc = " UTF-8, you can decode it first before using [`from_str`]."] pub fn with_resolver (reader : R , entity_resolver : E) -> Self { let mut reader = NsReader :: from_reader (reader) ; let config = reader . config_mut () ; config . expand_empty_elements = true ; Self :: new (IoReader { reader , buf : Vec :: new () , } , entity_resolver ,) } # [doc = " Create new deserializer that will copy data from the specified preconfigured reader"] # [doc = " into internal buffer and use the specified entity resolver."] # [doc = ""] # [doc = " Note, that config option [`Config::expand_empty_elements`] will be set to `true`."] # [doc = ""] # [doc = " [`Config::expand_empty_elements`]: crate::reader::Config::expand_empty_elements"] pub fn buffering_with_resolver (mut reader : NsReader < R > , entity_resolver : E) -> Self { let config = reader . config_mut () ; config . expand_empty_elements = true ; Self :: new (IoReader { reader , buf : Vec :: new () , } , entity_resolver ,) } }
};
}
