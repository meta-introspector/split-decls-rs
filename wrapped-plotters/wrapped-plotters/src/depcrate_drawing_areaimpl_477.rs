// Generated macro for impl_477 (impl)
macro_rules! Depcrate_drawing_areaimpl_477 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_477"}
// Dependencies: {}
impl < DB : DrawingBackend > From < DB > for DrawingArea < DB , Shift > { fn from (backend : DB) -> Self { Self :: with_rc_cell (Rc :: new (RefCell :: new (backend))) } }
};
}
