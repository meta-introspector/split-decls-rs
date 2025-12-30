// Generated macro for impl_195 (impl)
macro_rules! Depcrate_semaphoreimpl_195 {
() => {
// Module: crate::semaphore
// Provides: {"impl_195"}
// Dependencies: {}
impl SemaphoreGuardArc { # [doc = " Drops the guard _without_ releasing the acquired permit."] # [doc = " (Will still decrement the `Arc` reference count.)"] # [inline] pub fn forget (mut self) { drop (self . 0 . take ()) ; mem :: forget (self) ; } }
};
}
