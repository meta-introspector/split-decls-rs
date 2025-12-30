// Generated macro for cmd_captures (function)
macro_rules! Depcratecmd_captures {
() => {
// Module: crate
// Provides: {"cmd_captures"}
// Dependencies: {}
fn cmd_captures (args : & Args) -> Result < () > { let expr = try ! (args . parse_one ()) ; let prog = try ! (args . compiler () . only_utf8 (false) . compile (& [expr])) ; for (i , name) in prog . captures . iter () . enumerate () { match * name { None => println ! ("{}" , i) , Some (ref name) => println ! ("{}:{}" , i , name) , } } Ok (()) }
};
}
