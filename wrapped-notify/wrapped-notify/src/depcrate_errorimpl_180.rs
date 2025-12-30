// Generated macro for impl_180 (impl)
macro_rules! Depcrate_errorimpl_180 {
() => {
// Module: crate::error
// Provides: {"impl_180"}
// Dependencies: {}
impl < T > From < std :: sync :: mpsc :: SendError < T > > for Error { fn from (err : std :: sync :: mpsc :: SendError < T >) -> Self { Error :: generic (& format ! ("internal channel disconnect: {:?}" , err)) } }
};
}
