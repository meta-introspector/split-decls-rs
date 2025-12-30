// Generated macro for InitialHeader (struct)
macro_rules! Depcrate_packetInitialHeader {
() => {
// Module: crate::packet
// Provides: {"InitialHeader"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct InitialHeader { pub (crate) dst_cid : ConnectionId , pub (crate) src_cid : ConnectionId , pub (crate) token : Bytes , pub (crate) number : PacketNumber , pub (crate) version : u32 , }
};
}
