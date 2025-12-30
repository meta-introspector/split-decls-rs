// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_tests_fixtures_starwars_schemaimpl_1292 {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"impl_1292"}
// Dependencies: {}
# [graphql_object (context = Database)] # [doc = " The root query object of the schema"] impl Query { fn human (# [graphql (context)] database : & Database , # [graphql (description = "id of the human")] id : String ,) -> Option < & Human > { database . get_human (& id) } fn droid (# [graphql (context)] database : & Database , # [graphql (description = "id of the droid")] id : String ,) -> Option < & Droid > { database . get_droid (& id) } fn hero (# [graphql (context)] database : & Database , # [graphql (description = "If omitted, returns the hero of the whole saga. \
                                 If provided, returns the hero of that particular episode")] episode : Option < Episode > ,) -> Option < CharacterValue > { Some (database . get_hero (episode)) } }
};
}
