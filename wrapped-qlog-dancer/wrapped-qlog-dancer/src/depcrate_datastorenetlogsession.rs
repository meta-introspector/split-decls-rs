// Generated macro for NetlogSession (struct)
macro_rules! Depcrate_datastoreNetlogSession {
() => {
// Module: crate::datastore
// Provides: {"NetlogSession"}
// Dependencies: {}
# [derive (Tabled)] pub struct NetlogSession { # [tabled (rename = "ID")] session_id : i64 , # [tabled (rename = "Protocol")] application_proto : ApplicationProto , # [tabled (rename = "SNI")] host : String , start_time : u64 , }
};
}
