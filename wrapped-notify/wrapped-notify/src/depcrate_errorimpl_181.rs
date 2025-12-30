// Generated macro for impl_181 (impl)
macro_rules! Depcrate_errorimpl_181 {
() => {
// Module: crate::error
// Provides: {"impl_181"}
// Dependencies: {}
impl From < std :: sync :: mpsc :: RecvError > for Error { fn from (err : std :: sync :: mpsc :: RecvError) -> Self { Error :: generic (& format ! ("internal channel disconnect: {:?}" , err)) } }
};
}
