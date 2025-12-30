// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorsimpl_27 {
() => {
// Module: crate::errors
// Provides: {"impl_27"}
// Dependencies: {}
impl < T > From < :: std :: sync :: TryLockError < T > > for Error { fn from (_ : :: std :: sync :: TryLockError < T >) -> Self { Error :: TryLock } }
};
}
