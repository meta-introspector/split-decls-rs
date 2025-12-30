// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : StdError + Send + Sync + 'static > From < Box < T > > for Error { fn from (value : Box < T >) -> Self { Error :: Other (value) } }
};
}
