// Generated macro for impl_315 (impl)
macro_rules! Depcrate_errorimpl_315 {
() => {
// Module: crate::error
// Provides: {"impl_315"}
// Dependencies: {}
impl From < syn :: Error > for Error { fn from (e : syn :: Error) -> Self { Self { span : Some (e . span ()) , .. Self :: custom (e) } } }
};
}
