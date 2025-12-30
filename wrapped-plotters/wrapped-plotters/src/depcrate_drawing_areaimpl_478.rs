// Generated macro for impl_478 (impl)
macro_rules! Depcrate_drawing_areaimpl_478 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_478"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend > From < & 'a Rc < RefCell < DB > > > for DrawingArea < DB , Shift > { fn from (backend : & 'a Rc < RefCell < DB > >) -> Self { Self :: with_rc_cell (backend . clone ()) } }
};
}
