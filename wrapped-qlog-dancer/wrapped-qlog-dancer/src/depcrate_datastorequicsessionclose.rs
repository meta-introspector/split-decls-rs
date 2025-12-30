// Generated macro for QuicSessionClose (struct)
macro_rules! Depcrate_datastoreQuicSessionClose {
() => {
// Module: crate::datastore
// Provides: {"QuicSessionClose"}
// Dependencies: {}
# [derive (Debug , Default , Tabled)] pub struct QuicSessionClose { # [tabled (rename = "ID")] pub session_id : i64 , # [tabled (rename = "SNI")] pub sni : String , # [tabled (rename = "Error")] pub quic_error : i64 , # [tabled (rename = "Description")] pub quic_error_pretty : NaOption < String > , # [tabled (rename = "From peer")] pub from_peer : bool , # [tabled (rename = "Additional Details")] pub details : String , }
};
}
