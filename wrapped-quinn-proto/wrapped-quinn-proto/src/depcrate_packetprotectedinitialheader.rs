// Generated macro for ProtectedInitialHeader (struct)
macro_rules! Depcrate_packetProtectedInitialHeader {
() => {
// Module: crate::packet
// Provides: {"ProtectedInitialHeader"}
// Dependencies: {}
# [doc = " Header of an Initial packet, before decryption"] # [derive (Clone , Debug)] pub struct ProtectedInitialHeader { # [doc = " Destination Connection ID"] pub dst_cid : ConnectionId , # [doc = " Source Connection ID"] pub src_cid : ConnectionId , # [doc = " The position of a token in the packet buffer"] pub token_pos : Range < usize > , # [doc = " Length of the packet payload"] pub len : u64 , # [doc = " QUIC version"] pub version : u32 , }
};
}
