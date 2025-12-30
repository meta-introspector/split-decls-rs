// Generated macro for AltairSource (struct)
macro_rules! Depcrate_http_altair_sourceAltairSource {
() => {
// Module: crate::http::altair_source
// Provides: {"AltairSource"}
// Dependencies: {}
# [doc = " A builder for constructing an Altair HTML page."] # [derive (Default , Serialize)] pub struct AltairSource < 'a > { # [serde (default , skip_serializing_if = "Option::is_none")] title : Option < & 'a str > , # [serde (default , skip_serializing_if = "Option::is_none")] options : Option < serde_json :: Value > , }
};
}
