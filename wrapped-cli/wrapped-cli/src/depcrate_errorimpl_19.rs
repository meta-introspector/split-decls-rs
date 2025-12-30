// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl From < serde_norway :: Error > for Error { fn from (err : serde_norway :: Error) -> Self { Self :: Serializing (err . to_string ()) } }
};
}
