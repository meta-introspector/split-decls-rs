// Generated macro for impl_781 (impl)
macro_rules! Depcrate_data_structures_range_object_mapimpl_781 {
() => {
// Module: crate::data_structures::range_object_map
// Provides: {"impl_781"}
// Dependencies: {}
impl < T > IndexMut < Position > for RangeObjectMap < T > { fn index_mut (& mut self , pos : Position) -> & mut Self :: Output { & mut self . v [pos] . data } }
};
}
