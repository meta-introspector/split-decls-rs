// Generated macro for impl_432 (impl)
macro_rules! Depcrate_coordimpl_432 {
() => {
// Module: crate::coord
// Provides: {"impl_432"}
// Dependencies: {}
impl CoordTranslate for Shift { type From = BackendCoord ; fn translate (& self , from : & Self :: From) -> BackendCoord { (from . 0 + (self . 0) . 0 , from . 1 + (self . 0) . 1) } }
};
}
