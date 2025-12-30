// Generated macro for H2SessionClose (struct)
macro_rules! Depcrate_datastoreH2SessionClose {
() => {
// Module: crate::datastore
// Provides: {"H2SessionClose"}
// Dependencies: {}
# [derive (Debug , Default , Tabled)] pub struct H2SessionClose { # [tabled (rename = "ID")] pub session_id : i64 , # [tabled (rename = "SNI")] pub sni : String , # [tabled (rename = "Error")] pub net_err : i64 , # [tabled (rename = "Description")] pub net_err_pretty : NaOption < String > , # [tabled (rename = "Additional Details")] pub details : String , }
};
}
