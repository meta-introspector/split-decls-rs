// Generated macro for packet_type (function)
macro_rules! Depcrate_connection_qlogpacket_type {
() => {
// Module: crate::connection::qlog
// Provides: {"packet_type"}
// Dependencies: {}
# [cfg (feature = "qlog")] fn packet_type (space : SpaceId , is_0rtt : bool) -> PacketType { match space { SpaceId :: Initial => PacketType :: Initial , SpaceId :: Handshake => PacketType :: Handshake , SpaceId :: Data if is_0rtt => PacketType :: ZeroRtt , SpaceId :: Data => PacketType :: OneRtt , } }
};
}
