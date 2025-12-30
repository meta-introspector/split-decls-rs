// Generated macro for impl_1300 (impl)
macro_rules! Depcrate_tests_fixtures_starwars_schemaimpl_1300 {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"impl_1300"}
// Dependencies: {}
# [doc = " A humanoid creature in the Star Wars universe."] # [graphql_object (context = Database , impl = CharacterValue)] impl Human { # [doc = " The id of the human"] pub fn id (& self) -> & str { & self . id } # [doc = " The name of the human"] pub fn name (& self) -> Option < & str > { Some (self . name . as_str ()) } # [doc = " The friends of the human"] pub fn friends (& self , ctx : & Database) -> Vec < CharacterValue > { ctx . get_friends (& self . friend_ids) } # [doc = " Which movies they appear in"] pub fn appears_in (& self) -> & [Episode] { & self . appears_in } # [doc = " The home planet of the human"] pub fn home_planet (& self) -> & Option < String > { & self . home_planet } }
};
}
