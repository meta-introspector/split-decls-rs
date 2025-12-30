// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_tests_fixtures_starwars_schemaimpl_1299 {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"impl_1299"}
// Dependencies: {}
impl Human { pub fn new (id : & str , name : & str , friend_ids : & [& str] , appears_in : & [Episode] , secret_backstory : Option < & str > , home_planet : Option < & str > ,) -> Self { Self { id : id . into () , name : name . into () , friend_ids : friend_ids . iter () . copied () . map (Into :: into) . collect () , appears_in : appears_in . to_vec () , secret_backstory : secret_backstory . map (Into :: into) , home_planet : home_planet . map (Into :: into) , } } }
};
}
