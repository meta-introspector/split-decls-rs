// Generated macro for impl_18 (impl)
macro_rules! Depcrate_errorimpl_18 {
() => {
// Module: crate::error
// Provides: {"impl_18"}
// Dependencies: {}
impl From < serde_json :: Error > for Error { fn from (err : serde_json :: Error) -> Self { Self :: Serializing (err . to_string ()) } }
};
}
