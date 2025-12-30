// Generated macro for impl_629 (impl)
macro_rules! Depcrate_data_structures_range_object_mapimpl_629 {
() => {
// Module: crate::data_structures::range_object_map
// Provides: {"impl_629"}
// Dependencies: {}
impl < T > IndexMut < Position > for RangeObjectMap < T > { fn index_mut (& mut self , pos : Position) -> & mut Self :: Output { & mut self . v [pos] . data } }
};
}
