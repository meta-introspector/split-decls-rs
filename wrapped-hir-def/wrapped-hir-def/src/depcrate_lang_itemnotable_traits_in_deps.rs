// Generated macro for notable_traits_in_deps (function)
macro_rules! Depcrate_lang_itemnotable_traits_in_deps {
() => {
// Module: crate::lang_item
// Provides: {"notable_traits_in_deps"}
// Dependencies: {}
pub (crate) fn notable_traits_in_deps (db : & dyn DefDatabase , krate : Crate) -> Arc < [Arc < [TraitId] >] > { let _p = tracing :: info_span ! ("notable_traits_in_deps" , ? krate) . entered () ; Arc :: from_iter (db . transitive_deps (krate) . into_iter () . filter_map (| krate | db . crate_notable_traits (krate)) ,) }
};
}
