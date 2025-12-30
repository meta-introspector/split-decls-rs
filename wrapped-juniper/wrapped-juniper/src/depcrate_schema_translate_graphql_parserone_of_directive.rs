// Generated macro for one_of_directive (function)
macro_rules! Depcrate_schema_translate_graphql_parserone_of_directive {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"one_of_directive"}
// Dependencies: {}
# [doc = " Forms a [`@oneOf`] [`schema::Directive`]."] # [doc = ""] # [doc = " [`@oneOf`]: https://spec.graphql.org/September2025#sec--oneOf"] fn one_of_directive < 'a , T > () -> schema :: Directive < 'a , T > where T : schema :: Text < 'a > , { schema :: Directive { position : Pos :: default () , name : "oneOf" . into () , arguments : vec ! [] , } }
};
}
