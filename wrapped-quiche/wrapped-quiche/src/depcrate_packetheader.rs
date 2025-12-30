// Generated macro for Header (struct)
macro_rules! Depcrate_packetHeader {
() => {
// Module: crate::packet
// Provides: {"Header"}
// Dependencies: {}
# [doc = " A QUIC packet's header."] # [derive (Clone , PartialEq , Eq)] pub struct Header < 'a > { # [doc = " The type of the packet."] pub ty : Type , # [doc = " The version of the packet."] pub version : u32 , # [doc = " The destination connection ID of the packet."] pub dcid : ConnectionId < 'a > , # [doc = " The source connection ID of the packet."] pub scid : ConnectionId < 'a > , # [doc = " The packet number. It's only meaningful after the header protection is"] # [doc = " removed."] pub (crate) pkt_num : u64 , # [doc = " The length of the packet number. It's only meaningful after the header"] # [doc = " protection is removed."] pub (crate) pkt_num_len : usize , # [doc = " The address verification token of the packet. Only present in `Initial`"] # [doc = " and `Retry` packets."] pub token : Option < Vec < u8 > > , # [doc = " The list of versions in the packet. Only present in"] # [doc = " `VersionNegotiation` packets."] pub versions : Option < Vec < u32 > > , # [doc = " The key phase bit of the packet. It's only meaningful after the header"] # [doc = " protection is removed."] pub (crate) key_phase : bool , }
};
}
