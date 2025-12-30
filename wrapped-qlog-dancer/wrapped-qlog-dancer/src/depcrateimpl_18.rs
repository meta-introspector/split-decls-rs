// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl PacketType { pub fn from_qlog_packet_type (ty : & qlog :: events :: quic :: PacketType) -> Self { match ty { qlog :: events :: quic :: PacketType :: Initial => PacketType :: Initial , qlog :: events :: quic :: PacketType :: Handshake => PacketType :: Handshake , qlog :: events :: quic :: PacketType :: ZeroRtt => PacketType :: ZeroRtt , qlog :: events :: quic :: PacketType :: OneRtt => PacketType :: OneRtt , qlog :: events :: quic :: PacketType :: Retry => PacketType :: Retry , qlog :: events :: quic :: PacketType :: VersionNegotiation => PacketType :: VersionNegotiation , qlog :: events :: quic :: PacketType :: Unknown => PacketType :: Unknown , } } pub fn from_netlog_packet_header (header_format : & str , long_header_type : & Option < String > ,) -> Self { match header_format { "IETF_QUIC_LONG_HEADER_PACKET" => match long_header_type { Some (v) => match v . as_str () { "INITIAL" => PacketType :: Initial , "HANDSHAKE" => PacketType :: Handshake , _ => PacketType :: Unknown , } , None => PacketType :: Unknown , } , "IETF_QUIC_SHORT_HEADER_PACKET" => PacketType :: OneRtt , _ => PacketType :: Unknown , } } pub fn from_netlog_encryption_level (encryption_level : & str) -> Self { match encryption_level { "ENCRYPTION_INITIAL" => PacketType :: Initial , "ENCRYPTION_HANDSHAKE" => PacketType :: Handshake , "ENCRYPTION_ZERO_RTT" => PacketType :: ZeroRtt , "ENCRYPTION_FORWARD_SECURE" => PacketType :: OneRtt , _ => PacketType :: Unknown , } } }
};
}
