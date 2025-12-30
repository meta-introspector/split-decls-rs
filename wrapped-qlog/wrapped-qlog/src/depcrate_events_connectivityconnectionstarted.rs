// Generated macro for ConnectionStarted (struct)
macro_rules! Depcrate_events_connectivityConnectionStarted {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionStarted"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct ConnectionStarted { pub ip_version : Option < String > , pub src_ip : String , pub dst_ip : String , pub protocol : Option < String > , pub src_port : Option < u16 > , pub dst_port : Option < u16 > , pub src_cid : Option < Bytes > , pub dst_cid : Option < Bytes > , }
};
}
