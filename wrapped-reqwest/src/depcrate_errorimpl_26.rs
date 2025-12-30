// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorimpl_26 {
() => {
// Module: crate::error
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] impl From < crate :: error :: Error > for js_sys :: Error { fn from (err : Error) -> js_sys :: Error { js_sys :: Error :: new (& format ! ("{err}")) } }
};
}
