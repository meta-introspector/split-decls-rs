// Generated macro for PartialDecode (struct)
macro_rules! Depcrate_packetPartialDecode {
() => {
// Module: crate::packet
// Provides: {"PartialDecode"}
// Dependencies: {}
# [doc = " Decodes a QUIC packet's invariant header"] # [doc = ""] # [doc = " Due to packet number encryption, it is impossible to fully decode a header"] # [doc = " (which includes a variable-length packet number) without crypto context."] # [doc = " The crypto context (represented by the `Crypto` type in Quinn) is usually"] # [doc = " part of the `Connection`, or can be derived from the destination CID for"] # [doc = " Initial packets."] # [doc = ""] # [doc = " To cope with this, we decode the invariant header (which should be stable"] # [doc = " across QUIC versions), which gives us the destination CID and allows us"] # [doc = " to inspect the version and packet type (which depends on the version)."] # [doc = " This information allows us to fully decode and decrypt the packet."] # [cfg_attr (test , derive (Clone))] # [derive (Debug)] pub struct PartialDecode { plain_header : ProtectedHeader , buf : io :: Cursor < BytesMut > , }
};
}
