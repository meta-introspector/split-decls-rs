// Generated macro for impl_89 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_89 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'map , Key , Value , State > Clone for KeyValues < 'map , Key , Value , State > { fn clone (& self) -> KeyValues < 'map , Key , Value , State > { KeyValues { build_hasher : self . build_hasher , keys : self . keys , iter : self . iter . clone () , map : self . map , values : self . values , } } }
};
}
