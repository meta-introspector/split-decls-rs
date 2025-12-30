// Generated macro for impl_161 (impl)
macro_rules! Depcrate_errorimpl_161 {
() => {
// Module: crate::error
// Provides: {"impl_161"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: io :: Error > for DataError { fn from (e : std :: io :: Error) -> Self { log :: warn ! ("I/O error: {e}") ; DataErrorKind :: Io (e . kind ()) . into_error () } }
};
}
