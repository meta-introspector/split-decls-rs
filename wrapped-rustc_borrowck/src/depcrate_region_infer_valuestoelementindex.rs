// Generated macro for ToElementIndex (trait)
macro_rules! Depcrate_region_infer_valuesToElementIndex {
() => {
// Module: crate::region_infer::values
// Provides: {"ToElementIndex"}
// Dependencies: {}
pub (crate) trait ToElementIndex : Debug + Copy { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool ; fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool ; }
};
}
