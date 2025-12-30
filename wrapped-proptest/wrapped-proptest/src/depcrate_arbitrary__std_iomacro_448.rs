// Generated macro for macro_448 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_448 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_448"}
// Dependencies: {}
arbitrary ! (ErrorKind , Union < Just < Self >>; Union :: new ([NotFound , PermissionDenied , ConnectionRefused , ConnectionReset , ConnectionAborted , NotConnected , AddrInUse , AddrNotAvailable , BrokenPipe , AlreadyExists , WouldBlock , InvalidInput , InvalidData , TimedOut , WriteZero , Interrupted , Other , UnexpectedEof] . iter () . cloned () . map (Just))) ;
};
}
