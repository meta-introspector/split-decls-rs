// Generated macro for impl_98 (impl)
macro_rules! Depcrate_errorimpl_98 {
() => {
// Module: crate::error
// Provides: {"impl_98"}
// Dependencies: {}
impl From < UserError > for Error { fn from (src : UserError) -> Error { Error { kind : Kind :: User (src) , } } }
};
}
