// Generated macro for impl_182 (impl)
macro_rules! Depcrate_errorimpl_182 {
() => {
// Module: crate::error
// Provides: {"impl_182"}
// Dependencies: {}
impl < T > From < std :: sync :: PoisonError < T > > for Error { fn from (err : std :: sync :: PoisonError < T >) -> Self { Error :: generic (& format ! ("internal mutex poisoned: {:?}" , err)) } }
};
}
