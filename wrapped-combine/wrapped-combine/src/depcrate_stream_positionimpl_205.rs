// Generated macro for impl_205 (impl)
macro_rules! Depcrate_stream_positionimpl_205 {
() => {
// Module: crate::stream::position
// Provides: {"impl_205"}
// Dependencies: {}
impl < Item , Range , T > RangePositioner < Item , Range > for & '_ mut T where Item : Clone , Range : Clone + crate :: stream :: Range , T : ? Sized + RangePositioner < Item , Range > , { fn update_range (& mut self , range : & Range) { (* * self) . update_range (range) ; } }
};
}
