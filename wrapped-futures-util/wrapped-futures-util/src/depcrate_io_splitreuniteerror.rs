// Generated macro for ReuniteError (struct)
macro_rules! Depcrate_io_splitReuniteError {
() => {
// Module: crate::io::split
// Provides: {"ReuniteError"}
// Dependencies: {}
# [doc = " Error indicating a `ReadHalf<T>` and `WriteHalf<T>` were not two halves"] # [doc = " of a `AsyncRead + AsyncWrite`, and thus could not be `reunite`d."] pub struct ReuniteError < T > (pub ReadHalf < T > , pub WriteHalf < T >) ;
};
}
