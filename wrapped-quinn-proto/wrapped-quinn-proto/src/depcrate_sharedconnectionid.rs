// Generated macro for ConnectionId (struct)
macro_rules! Depcrate_sharedConnectionId {
() => {
// Module: crate::shared
// Provides: {"ConnectionId"}
// Dependencies: {}
# [doc = " Protocol-level identifier for a connection."] # [doc = ""] # [doc = " Mainly useful for identifying this connection's packets on the wire with tools like Wireshark."] # [derive (Clone , Copy , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct ConnectionId { # [doc = " length of CID"] len : u8 , # [doc = " CID in byte array"] bytes : [u8 ; MAX_CID_SIZE] , }
};
}
