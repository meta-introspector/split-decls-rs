// Generated macro for impl_74 (impl)
macro_rules! Depcrate_mockimpl_74 {
() => {
// Module: crate::mock
// Provides: {"impl_74"}
// Dependencies: {}
impl Drop for Handle { fn drop (& mut self) { let waker = futures :: task :: noop_waker () ; let mut cx = Context :: from_waker (& waker) ; assert ! (self . codec . shutdown (& mut cx) . is_ready ()) ; if let Ok (mut me) = self . codec . get_mut () . inner . lock () { me . closed = true ; if let Some (task) = me . rx_task . take () { task . wake () ; } } } }
};
}
