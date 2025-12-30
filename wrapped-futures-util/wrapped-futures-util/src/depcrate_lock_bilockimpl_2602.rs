// Generated macro for impl_2602 (impl)
macro_rules! Depcrate_lock_bilockimpl_2602 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2602"}
// Dependencies: {}
impl < T > BiLockGuard < '_ , T > { # [doc = " Get a mutable pinned reference to the locked value."] pub fn as_pin_mut (& mut self) -> Pin < & mut T > { unsafe { Pin :: new_unchecked (& mut * self . bilock . arc . value . as_ref () . unwrap () . get ()) } } }
};
}
