// Generated macro for impl_87 (impl)
macro_rules! Depcrate_streamimpl_87 {
() => {
// Module: crate::stream
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : AsLockedWrite + ? Sized > AsLockedWrite for Box < T > { type Write < 'w > = T :: Write < 'w > where Self : 'w ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { (* * self) . as_locked_write () } }
};
}
