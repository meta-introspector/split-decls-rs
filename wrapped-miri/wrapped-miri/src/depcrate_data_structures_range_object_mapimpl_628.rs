// Generated macro for impl_628 (impl)
macro_rules! Depcrate_data_structures_range_object_mapimpl_628 {
() => {
// Module: crate::data_structures::range_object_map
// Provides: {"impl_628"}
// Dependencies: {}
impl < T > Index < Position > for RangeObjectMap < T > { type Output = T ; fn index (& self , pos : Position) -> & Self :: Output { & self . v [pos] . data } }
};
}
