// Generated macro for get_non_default_port (function)
macro_rules! Depcrate_client_legacy_clientget_non_default_port {
() => {
// Module: crate::client::legacy::client
// Provides: {"get_non_default_port"}
// Dependencies: {}
fn get_non_default_port (uri : & Uri) -> Option < http :: uri :: Port < & str > > { match (uri . port () . map (| p | p . as_u16 ()) , is_schema_secure (uri)) { (Some (443) , true) => None , (Some (80) , false) => None , _ => uri . port () , } }
};
}
