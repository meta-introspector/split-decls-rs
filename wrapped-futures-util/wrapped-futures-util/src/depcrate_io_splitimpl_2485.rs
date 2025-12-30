// Generated macro for impl_2485 (impl)
macro_rules! Depcrate_io_splitimpl_2485 {
() => {
// Module: crate::io::split
// Provides: {"impl_2485"}
// Dependencies: {}
impl < T : Unpin > WriteHalf < T > { # [doc = " Attempts to put the two \"halves\" of a split `AsyncRead + AsyncWrite` back"] # [doc = " together. Succeeds only if the `ReadHalf<T>` and `WriteHalf<T>` are"] # [doc = " a matching pair originating from the same call to `AsyncReadExt::split`."] pub fn reunite (self , other : ReadHalf < T >) -> Result < T , ReuniteError < T > > { other . reunite (self) } }
};
}
