// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl Config { pub fn build (args : & [String]) -> Result < Config , & 'static str > { if args . len () < 3 { return Err ("not enough arguments") ; } let query = args [1] . clone () ; let file_path = args [2] . clone () ; Ok (Config { query , file_path }) } }
};
}
