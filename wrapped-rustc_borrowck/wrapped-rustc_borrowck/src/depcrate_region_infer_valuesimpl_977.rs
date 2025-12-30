// Generated macro for impl_977 (impl)
macro_rules! Depcrate_region_infer_valuesimpl_977 {
() => {
// Module: crate::region_infer::values
// Provides: {"impl_977"}
// Dependencies: {}
impl ToElementIndex for RegionVid { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { values . free_regions . insert (row , self) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { values . free_regions . contains (row , self) } }
};
}
