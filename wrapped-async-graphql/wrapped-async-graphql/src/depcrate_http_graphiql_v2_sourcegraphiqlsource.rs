// Generated macro for GraphiQLSource (struct)
macro_rules! Depcrate_http_graphiql_v2_sourceGraphiQLSource {
() => {
// Module: crate::http::graphiql_v2_source
// Provides: {"GraphiQLSource"}
// Dependencies: {}
# [doc = " A builder for constructing a GraphiQL (v2) HTML page."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::http::*;"] # [doc = ""] # [doc = " GraphiQLSource::build()"] # [doc = "     .endpoint(\"/\")"] # [doc = "     .subscription_endpoint(\"/ws\")"] # [doc = "     .header(\"Authorization\", \"Bearer [token]\")"] # [doc = "     .ws_connection_param(\"token\", \"[token]\")"] # [doc = "     .credentials(Credentials::Include)"] # [doc = "     .finish();"] # [doc = " ```"] # [derive (Default , Serialize)] pub struct GraphiQLSource < 'a > { endpoint : & 'a str , subscription_endpoint : Option < & 'a str > , version : GraphiQLVersion < 'a > , headers : Option < HashMap < & 'a str , & 'a str > > , ws_connection_params : Option < HashMap < & 'a str , & 'a str > > , title : Option < & 'a str > , credentials : Credentials , plugins : & 'a [GraphiQLPlugin < 'a >] , }
};
}
