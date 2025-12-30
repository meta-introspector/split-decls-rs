// Generated macro for impl_211 (impl)
macro_rules! Depcrate_errorimpl_211 {
() => {
// Module: crate::error
// Provides: {"impl_211"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < io :: Error > for Error { fn from (e : io :: Error) -> Self { Error :: Io (e . to_string ()) } }
};
}
