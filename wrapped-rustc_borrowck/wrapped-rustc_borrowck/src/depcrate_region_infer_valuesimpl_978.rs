// Generated macro for impl_978 (impl)
macro_rules! Depcrate_region_infer_valuesimpl_978 {
() => {
// Module: crate::region_infer::values
// Provides: {"impl_978"}
// Dependencies: {}
impl ToElementIndex for ty :: PlaceholderRegion { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . contains (row , index) } }
};
}
