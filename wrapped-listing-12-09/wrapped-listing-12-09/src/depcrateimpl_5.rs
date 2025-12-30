// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Config { fn build (args : & [String]) -> Result < Config , & 'static str > { if args . len () < 3 { return Err ("not enough arguments") ; } let query = args [1] . clone () ; let file_path = args [2] . clone () ; Ok (Config { query , file_path }) } }
};
}
