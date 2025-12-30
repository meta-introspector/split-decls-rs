// Generated macro for testing (module)
macro_rules! Depcratetesting {
() => {
// Module: crate
// Provides: {"testing"}
// Dependencies: {}
# [doc (hidden)] pub mod testing { use super :: * ; use crate :: events :: quic :: PacketType ; pub fn make_pkt_hdr (packet_type : PacketType) -> PacketHeader { let scid = [0x7e , 0x37 , 0xe4 , 0xdc , 0xc6 , 0x68 , 0x2d , 0xa8] ; let dcid = [0x36 , 0xce , 0x10 , 0x4e , 0xee , 0x50 , 0x10 , 0x1c] ; PacketHeader :: new (packet_type , Some (0) , None , None , None , Some (0x0000_0001) , Some (& scid) , Some (& dcid) ,) } pub fn make_trace () -> Trace { Trace :: new (VantagePoint { name : None , ty : VantagePointType :: Server , flow : None , } , Some ("Quiche qlog trace" . to_string ()) , Some ("Quiche qlog trace description" . to_string ()) , Some (Configuration { time_offset : Some (0.0) , original_uris : None , }) , None ,) } pub fn make_trace_seq () -> TraceSeq { TraceSeq :: new (VantagePoint { name : None , ty : VantagePointType :: Server , flow : None , } , Some ("Quiche qlog trace" . to_string ()) , Some ("Quiche qlog trace description" . to_string ()) , Some (Configuration { time_offset : Some (0.0) , original_uris : None , }) , None ,) } }
};
}
