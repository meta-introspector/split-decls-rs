// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
impl From < core :: num :: TryFromIntError > for Error { fn from (_ : core :: num :: TryFromIntError) -> Self { WIN32_ERROR (ERROR_INVALID_DATA) . into () } }
};
}
