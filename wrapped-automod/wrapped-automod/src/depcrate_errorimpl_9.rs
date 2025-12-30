// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (err : io :: Error) -> Self { Error :: Io (err) } }
};
}
