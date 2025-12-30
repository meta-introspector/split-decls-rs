// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let crate_dir = std :: env :: args () . skip (1) . next () . unwrap () ; let expect = std :: path :: Path :: new (& crate_dir) . join ("src/main.rs") ; let bt = backtrace :: Backtrace :: new () ; println ! ("{bt:?}") ; let mut found_main = false ; for frame in bt . frames () { let symbols = frame . symbols () ; if symbols . is_empty () { continue ; } if let Some (name) = symbols [0] . name () { let name = format ! ("{name:#}") ; if name == "debuglink::main" { found_main = true ; let filename = symbols [0] . filename () . unwrap () ; assert_eq ! (filename , expect) ; break ; } } } assert ! (found_main) ; }
};
}
