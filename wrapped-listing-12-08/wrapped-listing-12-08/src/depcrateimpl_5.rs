// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Config { fn new (args : & [String]) -> Config { if args . len () < 3 { panic ! ("not enough arguments") ; } let query = args [1] . clone () ; let file_path = args [2] . clone () ; Config { query , file_path } } }
};
}
