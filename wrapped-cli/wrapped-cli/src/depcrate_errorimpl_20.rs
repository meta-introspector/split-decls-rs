// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl From < toml_edit :: ser :: Error > for Error { fn from (err : toml_edit :: ser :: Error) -> Self { Self :: Serializing (err . to_string ()) } }
};
}
