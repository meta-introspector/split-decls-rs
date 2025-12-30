// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut args = env :: args () . skip (1) ; let next = args . next () ; if next . is_none () { return help () ; } match & next . unwrap () [..] { "spawn-emulator" => spawn_emulator (& args . next () . unwrap () , Path :: new (& args . next () . unwrap ()) , Path :: new (& args . next () . unwrap ()) , args . next () . map (| s | s . into ()) ,) , "push" => push (Path :: new (& args . next () . unwrap ())) , "run" => run (args . next () . and_then (| count | count . parse () . ok ()) . unwrap () , args . next () . unwrap () , args . collect () ,) , "help" | "-h" | "--help" => help () , cmd => { println ! ("unknown command: {}" , cmd) ; help () ; } } }
};
}
