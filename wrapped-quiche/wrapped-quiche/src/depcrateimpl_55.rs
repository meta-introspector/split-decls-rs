// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl Error { fn to_wire (self) -> u64 { match self { Error :: Done => WireErrorCode :: NoError as u64 , Error :: InvalidFrame => WireErrorCode :: FrameEncodingError as u64 , Error :: InvalidStreamState (..) => WireErrorCode :: StreamStateError as u64 , Error :: InvalidTransportParam => WireErrorCode :: TransportParameterError as u64 , Error :: FlowControl => WireErrorCode :: FlowControlError as u64 , Error :: StreamLimit => WireErrorCode :: StreamLimitError as u64 , Error :: IdLimit => WireErrorCode :: ConnectionIdLimitError as u64 , Error :: FinalSize => WireErrorCode :: FinalSizeError as u64 , Error :: CryptoBufferExceeded => WireErrorCode :: CryptoBufferExceeded as u64 , Error :: KeyUpdate => WireErrorCode :: KeyUpdateError as u64 , _ => WireErrorCode :: ProtocolViolation as u64 , } } # [cfg (feature = "ffi")] fn to_c (self) -> libc :: ssize_t { match self { Error :: Done => - 1 , Error :: BufferTooShort => - 2 , Error :: UnknownVersion => - 3 , Error :: InvalidFrame => - 4 , Error :: InvalidPacket => - 5 , Error :: InvalidState => - 6 , Error :: InvalidStreamState (_) => - 7 , Error :: InvalidTransportParam => - 8 , Error :: CryptoFail => - 9 , Error :: TlsFail => - 10 , Error :: FlowControl => - 11 , Error :: StreamLimit => - 12 , Error :: FinalSize => - 13 , Error :: CongestionControl => - 14 , Error :: StreamStopped { .. } => - 15 , Error :: StreamReset { .. } => - 16 , Error :: IdLimit => - 17 , Error :: OutOfIdentifiers => - 18 , Error :: KeyUpdate => - 19 , Error :: CryptoBufferExceeded => - 20 , Error :: InvalidAckRange => - 21 , Error :: OptimisticAckDetected => - 22 , } } }
};
}
