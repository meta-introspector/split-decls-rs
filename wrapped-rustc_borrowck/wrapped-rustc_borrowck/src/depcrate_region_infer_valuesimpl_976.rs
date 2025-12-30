// Generated macro for impl_976 (impl)
macro_rules! Depcrate_region_infer_valuesimpl_976 {
() => {
// Module: crate::region_infer::values
// Provides: {"impl_976"}
// Dependencies: {}
impl ToElementIndex for Location { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . contains (row , index) } }
};
}
