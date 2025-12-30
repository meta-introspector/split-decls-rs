// Generated macro for maybe_sort_entries (function)
macro_rules! Depcrate_store_impls_dynamic_itermaybe_sort_entries {
() => {
// Module: crate::store_impls::dynamic::iter
// Provides: {"maybe_sort_entries"}
// Dependencies: {}
fn maybe_sort_entries (index : & handle :: IndexLookup , order : Ordering) -> Option < Vec < EntryForOrdering > > { let mut order : Vec < _ > = match order { Ordering :: PackLexicographicalThenLooseLexicographical => return None , Ordering :: PackAscendingOffsetThenLooseLexicographical => match & index . file { SingleOrMultiIndex :: Single { index , .. } => index . iter () . enumerate () . map (| (idx , e) | EntryForOrdering { pack_offset : e . pack_offset , entry_index : idx as u32 , pack_index : 0 , }) . collect () , SingleOrMultiIndex :: Multi { index , .. } => index . iter () . enumerate () . map (| (idx , e) | EntryForOrdering { pack_offset : e . pack_offset , entry_index : idx as u32 , pack_index : { debug_assert ! (e . pack_index < PackId :: max_packs_in_multi_index () , "this shows the relation between u16 and pack_index (u32) and why this is OK") ; e . pack_index as u16 } , }) . collect () , } , } ; order . sort_by (| a , b | { a . pack_index . cmp (& b . pack_index) . then_with (| | a . pack_offset . cmp (& b . pack_offset)) }) ; Some (order) }
};
}
