// Generated macro for impl_97 (impl)
macro_rules! Depcrate_errorimpl_97 {
() => {
// Module: crate::error
// Provides: {"impl_97"}
// Dependencies: {}
impl From < SendError > for Error { fn from (src : SendError) -> Error { match src { SendError :: User (e) => e . into () , SendError :: Connection (e) => e . into () , } } }
};
}
