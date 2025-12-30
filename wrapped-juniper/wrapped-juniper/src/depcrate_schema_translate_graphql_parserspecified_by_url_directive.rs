// Generated macro for specified_by_url_directive (function)
macro_rules! Depcrate_schema_translate_graphql_parserspecified_by_url_directive {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"specified_by_url_directive"}
// Dependencies: {}
# [doc = " Forms a `@specifiedBy(url:)` [`schema::Directive`] out of the provided `url`."] # [doc = ""] # [doc = " [`@specifiedBy(url:)`]: https://spec.graphql.org/September2025#sec--specifiedBy"] fn specified_by_url_directive < 'a , T > (url : & str) -> schema :: Directive < 'a , T > where T : schema :: Text < 'a > , { schema :: Directive { position : Pos :: default () , name : "specifiedBy" . into () , arguments : vec ! [("url" . into () , schema :: Value :: String (url . into ()))] , } }
};
}
