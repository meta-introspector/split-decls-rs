// Generated macro for AltairAuthorizationProviderInput (enum)
macro_rules! Depcrate_http_altair_sourceAltairAuthorizationProviderInput {
() => {
// Module: crate::http::altair_source
// Provides: {"AltairAuthorizationProviderInput"}
// Dependencies: {}
# [doc = " Altair authorization provider input"] # [derive (Serialize , Deserialize , JsonSchema)] # [serde (tag = "type" , content = "data")] pub enum AltairAuthorizationProviderInput { # [doc = " Api key authorization"] # [serde (rename = "api-key")] ApiKey { # [doc = " Header name"] header_name : String , # [doc = " Header value"] header_value : String , } , # [doc = " Basic authorization"] # [serde (rename = "basic")] Basic { # [doc = " Password"] password : String , # [doc = " Username"] username : String , } , # [doc = " Bearer token authorization"] # [serde (rename = "bearer")] Bearer { # [doc = " Token"] token : String , } , # [doc = " OAuth2 access token authorization"] # [serde (rename = "oauth2")] OAuth2 { # [doc = " Access token response"] access_token_response : String , } , }
};
}
