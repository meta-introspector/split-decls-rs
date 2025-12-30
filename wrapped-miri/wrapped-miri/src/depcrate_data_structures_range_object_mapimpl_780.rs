// Generated macro for impl_780 (impl)
macro_rules! Depcrate_data_structures_range_object_mapimpl_780 {
() => {
// Module: crate::data_structures::range_object_map
// Provides: {"impl_780"}
// Dependencies: {}
impl < T > Index < Position > for RangeObjectMap < T > { type Output = T ; fn index (& self , pos : Position) -> & Self :: Output { & self . v [pos] . data } }
};
}
