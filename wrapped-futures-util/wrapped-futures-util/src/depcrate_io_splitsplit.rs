// Generated macro for split (function)
macro_rules! Depcrate_io_splitsplit {
() => {
// Module: crate::io::split
// Provides: {"split"}
// Dependencies: {}
pub (super) fn split < T : AsyncRead + AsyncWrite > (t : T) -> (ReadHalf < T > , WriteHalf < T >) { let (a , b) = BiLock :: new (t) ; (ReadHalf { handle : a } , WriteHalf { handle : b }) }
};
}
