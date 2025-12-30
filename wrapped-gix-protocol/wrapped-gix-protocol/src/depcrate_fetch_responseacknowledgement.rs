// Generated macro for Acknowledgement (enum)
macro_rules! Depcrate_fetch_responseAcknowledgement {
() => {
// Module: crate::fetch::response
// Provides: {"Acknowledgement"}
// Dependencies: {}
# [doc = " An 'ACK' line received from the server."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Acknowledgement { # [doc = " The contained `id` is in common."] Common (gix_hash :: ObjectId) , # [doc = " The server is ready to receive more lines."] Ready , # [doc = " The server isn't ready yet."] Nak , }
};
}
