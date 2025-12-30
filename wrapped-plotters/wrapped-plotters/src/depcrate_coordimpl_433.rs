// Generated macro for impl_433 (impl)
macro_rules! Depcrate_coordimpl_433 {
() => {
// Module: crate::coord
// Provides: {"impl_433"}
// Dependencies: {}
impl ReverseCoordTranslate for Shift { fn reverse_translate (& self , input : BackendCoord) -> Option < BackendCoord > { Some ((input . 0 - (self . 0) . 0 , input . 1 - (self . 0) . 1)) } }
};
}
