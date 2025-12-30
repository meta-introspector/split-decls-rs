// Generated macro for impl_134 (impl)
macro_rules! Depcrate_rwlockimpl_134 {
() => {
// Module: crate::rwlock
// Provides: {"impl_134"}
// Dependencies: {}
impl < T > Deref for RwLockReadGuardArc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { let arc = Self :: inner_arc (self) ; & * arc . value . get () } } }
};
}
