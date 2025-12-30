// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl From < String > for Error { fn from (message : String) -> Self { Box :: new (StringTypedError { message , source : None , }) . into () } }
};
}
