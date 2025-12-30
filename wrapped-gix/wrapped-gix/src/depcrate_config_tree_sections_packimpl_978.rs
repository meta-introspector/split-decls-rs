// Generated macro for impl_978 (impl)
macro_rules! Depcrate_config_tree_sections_packimpl_978 {
() => {
// Module: crate::config::tree::sections::pack
// Provides: {"impl_978"}
// Dependencies: {}
impl Pack { # [doc = " The `pack.threads` key."] pub const THREADS : keys :: UnsignedInteger = keys :: UnsignedInteger :: new_unsigned_integer ("threads" , & config :: Tree :: PACK) . with_deviation ("Leaving this key unspecified uses all available cores, instead of 1") ; # [doc = " The `pack.indexVersion` key."] pub const INDEX_VERSION : IndexVersion = IndexVersion :: new_with_validate ("indexVersion" , & config :: Tree :: PACK , validate :: IndexVersion) ; }
};
}
