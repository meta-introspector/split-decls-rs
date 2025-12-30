// Generated macro for impl_889 (impl)
macro_rules! Depcrate_packetimpl_889 {
() => {
// Module: crate::packet
// Provides: {"impl_889"}
// Dependencies: {}
impl Type { pub (crate) fn from_epoch (e : Epoch) -> Type { match e { Epoch :: Initial => Type :: Initial , Epoch :: Handshake => Type :: Handshake , Epoch :: Application => Type :: Short , } } pub (crate) fn to_epoch (self) -> Result < Epoch > { match self { Type :: Initial => Ok (Epoch :: Initial) , Type :: ZeroRTT => Ok (Epoch :: Application) , Type :: Handshake => Ok (Epoch :: Handshake) , Type :: Short => Ok (Epoch :: Application) , _ => Err (Error :: InvalidPacket) , } } # [cfg (feature = "qlog")] pub (crate) fn to_qlog (self) -> qlog :: events :: quic :: PacketType { match self { Type :: Initial => qlog :: events :: quic :: PacketType :: Initial , Type :: Retry => qlog :: events :: quic :: PacketType :: Retry , Type :: Handshake => qlog :: events :: quic :: PacketType :: Handshake , Type :: ZeroRTT => qlog :: events :: quic :: PacketType :: ZeroRtt , Type :: VersionNegotiation => qlog :: events :: quic :: PacketType :: VersionNegotiation , Type :: Short => qlog :: events :: quic :: PacketType :: OneRtt , } } }
};
}
