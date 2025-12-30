// Generated macro for __impl_iterable_dynamic_data_provider (macro)
macro_rules! Depcrate_dynutil__impl_iterable_dynamic_data_provider {
() => {
// Module: crate::dynutil
// Provides: {"__impl_iterable_dynamic_data_provider"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __impl_iterable_dynamic_data_provider { ($ provider : ty , [$ ($ (# [$ cfg : meta]) ? $ struct_m : ty) ,+,] , $ dyn_m : path) => { impl $ crate :: IterableDynamicDataProvider <$ dyn_m > for $ provider { fn iter_ids_for_marker (& self , marker : $ crate :: DataMarkerInfo) -> Result < alloc :: collections :: BTreeSet <$ crate :: DataIdentifierCow <'_ >>, $ crate :: DataError > { match marker . id . hashed () { $ ($ (# [$ cfg]) ? h if h == <$ struct_m as $ crate :: DataMarker >:: INFO . id . hashed () => { $ crate :: IterableDataProvider ::<$ struct_m >:: iter_ids (self) }) +, _ => Err ($ crate :: DataErrorKind :: MarkerNotFound . with_marker (marker)) } } } } }
};
}
