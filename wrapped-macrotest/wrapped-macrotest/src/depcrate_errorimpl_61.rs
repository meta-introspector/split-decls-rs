// Generated macro for impl_61 (impl)
macro_rules! Depcrate_errorimpl_61 {
() => {
// Module: crate::error
// Provides: {"impl_61"}
// Dependencies: {}
impl From < toml :: ser :: Error > for Error { fn from (e : toml :: ser :: Error) -> Self { Error :: TomlSer (e) } }
};
}
