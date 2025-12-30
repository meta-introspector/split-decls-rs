// Generated macro for cmd_ast (function)
macro_rules! Depcratecmd_ast {
() => {
// Module: crate
// Provides: {"cmd_ast"}
// Dependencies: {}
fn cmd_ast (args : & Args) -> Result < () > { println ! ("{:#?}" , try ! (args . parse_one ())) ; Ok (()) }
};
}
