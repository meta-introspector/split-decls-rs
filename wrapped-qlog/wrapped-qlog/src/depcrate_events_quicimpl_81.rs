// Generated macro for impl_81 (impl)
macro_rules! Depcrate_events_quicimpl_81 {
() => {
// Module: crate::events::quic
// Provides: {"impl_81"}
// Dependencies: {}
impl PacketHeader { # [allow (clippy :: too_many_arguments)] # [doc = " Creates a new PacketHeader."] pub fn new (packet_type : PacketType , packet_number : Option < u64 > , flags : Option < u8 > , token : Option < Token > , length : Option < u16 > , version : Option < u32 > , scid : Option < & [u8] > , dcid : Option < & [u8] > ,) -> Self { let (scil , scid) = match scid { Some (cid) => (Some (cid . len () as u8) , Some (format ! ("{}" , HexSlice :: new (& cid))) ,) , None => (None , None) , } ; let (dcil , dcid) = match dcid { Some (cid) => (Some (cid . len () as u8) , Some (format ! ("{}" , HexSlice :: new (& cid))) ,) , None => (None , None) , } ; let version = version . map (| v | format ! ("{v:x?}")) ; PacketHeader { packet_type , packet_number , flags , token , length , version , scil , dcil , scid , dcid , } } # [doc = " Creates a new PacketHeader."] # [doc = ""] # [doc = " Once a QUIC connection has formed, version, dcid and scid are stable, so"] # [doc = " there are space benefits to not logging them in every packet, especially"] # [doc = " PacketType::OneRtt."] pub fn with_type (ty : PacketType , packet_number : Option < u64 > , version : Option < u32 > , scid : Option < & [u8] > , dcid : Option < & [u8] > ,) -> Self { match ty { PacketType :: OneRtt => PacketHeader :: new (ty , packet_number , None , None , None , None , None , None ,) , _ => PacketHeader :: new (ty , packet_number , None , None , None , version , scid , dcid ,) , } } }
};
}
