// Generated macro for Character (trait)
macro_rules! Depcrate_tests_fixtures_starwars_schemaCharacter {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"Character"}
// Dependencies: {}
# [graphql_interface (for = [Human , Droid] , context = Database)] # [doc = " A character in the Star Wars Trilogy"] pub trait Character { # [doc = " The id of the character"] fn id (& self) -> & str ; # [doc = " The name of the character"] fn name (& self) -> Option < & str > ; # [doc = " The friends of the character"] fn friends (& self , ctx : & Database) -> Vec < CharacterValue > ; # [doc = " Which movies they appear in"] fn appears_in (& self) -> & [Episode] ; # [graphql (ignore)] fn friends_ids (& self) -> & [String] ; }
};
}
