// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_tests_fixtures_starwars_schemaimpl_1303 {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"impl_1303"}
// Dependencies: {}
# [doc = " A mechanical creature in the Star Wars universe."] # [graphql_object (context = Database , impl = CharacterValue)] impl Droid { # [doc = " The id of the droid"] pub fn id (& self) -> & str { & self . id } # [doc = " The name of the droid"] pub fn name (& self) -> Option < & str > { Some (self . name . as_str ()) } # [doc = " The friends of the droid"] pub fn friends (& self , ctx : & Database) -> Vec < CharacterValue > { ctx . get_friends (& self . friend_ids) } # [doc = " Which movies they appear in"] pub fn appears_in (& self) -> & [Episode] { & self . appears_in } # [doc = " The primary function of the droid"] pub fn primary_function (& self) -> & Option < String > { & self . primary_function } }
};
}
