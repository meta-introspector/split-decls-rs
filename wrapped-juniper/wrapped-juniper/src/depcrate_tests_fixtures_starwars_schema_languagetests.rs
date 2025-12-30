// Generated macro for tests (module)
macro_rules! Depcrate_tests_fixtures_starwars_schema_languagetests {
() => {
// Module: crate::tests::fixtures::starwars::schema_language
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use pretty_assertions :: assert_eq ; use crate :: { schema :: model :: RootNode , tests :: fixtures :: starwars :: { schema :: { Database , Query } , schema_language :: STATIC_GRAPHQL_SCHEMA_DEFINITION , } , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; # [test] fn dynamic_schema_language_matches_static () { let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; # [cfg (windows)] let expected = STATIC_GRAPHQL_SCHEMA_DEFINITION . replace ("\r\n" , "\n") ; # [cfg (not (windows))] let expected = STATIC_GRAPHQL_SCHEMA_DEFINITION ; assert_eq ! (schema . as_sdl () , expected) ; } }
};
}
