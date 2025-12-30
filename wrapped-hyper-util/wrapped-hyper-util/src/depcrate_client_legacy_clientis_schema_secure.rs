// Generated macro for is_schema_secure (function)
macro_rules! Depcrate_client_legacy_clientis_schema_secure {
() => {
// Module: crate::client::legacy::client
// Provides: {"is_schema_secure"}
// Dependencies: {}
fn is_schema_secure (uri : & Uri) -> bool { uri . scheme_str () . map (| scheme_str | matches ! (scheme_str , "wss" | "https")) . unwrap_or_default () }
};
}
