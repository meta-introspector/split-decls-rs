// Generated macro for impl_209 (impl)
macro_rules! Depcrate_stream_positionimpl_209 {
() => {
// Module: crate::stream::position
// Provides: {"impl_209"}
// Dependencies: {}
impl < Item , Range > RangePositioner < Item , Range > for IndexPositioner where Item : Clone , Range : Clone + crate :: stream :: Range , { fn update_range (& mut self , range : & Range) { self . 0 += range . len () } }
};
}
