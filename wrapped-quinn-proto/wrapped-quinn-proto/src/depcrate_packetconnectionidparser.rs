// Generated macro for ConnectionIdParser (trait)
macro_rules! Depcrate_packetConnectionIdParser {
() => {
// Module: crate::packet
// Provides: {"ConnectionIdParser"}
// Dependencies: {}
# [doc = " Parse connection id in short header packet"] pub trait ConnectionIdParser { # [doc = " Parse a connection id from given buffer"] fn parse (& self , buf : & mut dyn Buf) -> Result < ConnectionId , PacketDecodeError > ; }
};
}
