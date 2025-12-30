// Generated macro for impl_2483 (impl)
macro_rules! Depcrate_io_splitimpl_2483 {
() => {
// Module: crate::io::split
// Provides: {"impl_2483"}
// Dependencies: {}
impl < T : Unpin > ReadHalf < T > { # [doc = " Attempts to put the two \"halves\" of a split `AsyncRead + AsyncWrite` back"] # [doc = " together. Succeeds only if the `ReadHalf<T>` and `WriteHalf<T>` are"] # [doc = " a matching pair originating from the same call to `AsyncReadExt::split`."] pub fn reunite (self , other : WriteHalf < T >) -> Result < T , ReuniteError < T > > { self . handle . reunite (other . handle) . map_err (| err | ReuniteError (Self { handle : err . 0 } , WriteHalf { handle : err . 1 })) } }
};
}
