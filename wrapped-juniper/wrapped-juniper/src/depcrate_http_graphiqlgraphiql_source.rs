// Generated macro for graphiql_source (function)
macro_rules! Depcrate_http_graphiqlgraphiql_source {
() => {
// Module: crate::http::graphiql
// Provides: {"graphiql_source"}
// Dependencies: {}
# [doc = " Generate the HTML source to show a GraphiQL interface"] # [doc = ""] # [doc = " The subscriptions endpoint URL can optionally be provided. For example:"] # [doc = ""] # [doc = " ```"] # [doc = " # use juniper::http::graphiql::graphiql_source;"] # [doc = " let graphiql = graphiql_source(\"/graphql\", Some(\"/subscriptions\"));"] # [doc = " ```"] pub fn graphiql_source (graphql_endpoint_url : & str , subscriptions_endpoint_url : Option < & str > ,) -> String { include_str ! ("graphiql.html") . replace ("<!-- inject -->" , & format ! ("
      const JUNIPER_URL = '{juniper_url}';
      const JUNIPER_SUBSCRIPTIONS_URL = '{juniper_subscriptions_url}';

{grahiql_js}

            " , juniper_url = graphql_endpoint_url , juniper_subscriptions_url = subscriptions_endpoint_url . unwrap_or_default () , grahiql_js = include_str ! ("graphiql.js") ,) ,) }
};
}
