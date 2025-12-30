// Generated macro for impl_96 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_96 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_96"}
// Dependencies: {}
impl < Key , Value , State > KeyValuesMut < '_ , Key , Value , State > { # [doc = " Creates an iterator that yields mutable references to all key-value pairs of a multimap."] # [must_use] pub fn iter (& self) -> KeyValues < '_ , Key , Value , State > { KeyValues { build_hasher : self . build_hasher , keys : self . keys , iter : self . iter . clone () , map : self . map , values : unsafe { & * self . values } , } } }
};
}
