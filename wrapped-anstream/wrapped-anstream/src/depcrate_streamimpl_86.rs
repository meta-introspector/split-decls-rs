// Generated macro for impl_86 (impl)
macro_rules! Depcrate_streamimpl_86 {
() => {
// Module: crate::stream
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : AsLockedWrite + ? Sized > AsLockedWrite for & mut T { type Write < 'w > = T :: Write < 'w > where Self : 'w ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { (* * self) . as_locked_write () } }
};
}
