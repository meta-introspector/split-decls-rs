// Generated macro for deprecation_directive (function)
macro_rules! Depcrate_schema_translate_graphql_parserdeprecation_directive {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"deprecation_directive"}
// Dependencies: {}
# [doc = " Forms a [`@deprecated(reason:)`] [`schema::Directive`] out of the provided"] # [doc = " [`meta::DeprecationStatus`]."] # [doc = ""] # [doc = " [`@deprecated(reason:)`]: https://spec.graphql.org/September2025#sec--deprecated"] fn deprecation_directive < 'a , T > (status : & meta :: DeprecationStatus ,) -> Option < schema :: Directive < 'a , T > > where T : schema :: Text < 'a > , { match status { meta :: DeprecationStatus :: Current => None , meta :: DeprecationStatus :: Deprecated (reason) => Some (schema :: Directive { position : Pos :: default () , name : "deprecated" . into () , arguments : reason . as_ref () . map (| rsn | vec ! [("reason" . into () , schema :: Value :: String (rsn . as_str () . into ()))]) . unwrap_or_default () , }) , } }
};
}
