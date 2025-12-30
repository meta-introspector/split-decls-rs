// Generated macro for impl_94 (impl)
macro_rules! Depcrate_globalimpl_94 {
() => {
// Module: crate::global
// Provides: {"impl_94"}
// Dependencies: {}
impl < F : ? Sized + BlockFn > Deref for GlobalBlock < F > { type Target = Block < F > ; # [inline] fn deref (& self) -> & Self :: Target { let ptr : NonNull < Self > = NonNull :: from (self) ; let ptr : NonNull < Block < F > > = ptr . cast () ; unsafe { ptr . as_ref () } } }
};
}
