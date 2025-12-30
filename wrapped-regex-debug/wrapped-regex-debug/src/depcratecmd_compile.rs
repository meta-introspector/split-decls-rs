// Generated macro for cmd_compile (function)
macro_rules! Depcratecmd_compile {
() => {
// Module: crate
// Provides: {"cmd_compile"}
// Dependencies: {}
fn cmd_compile (args : & Args) -> Result < () > { let exprs = try ! (args . parse_many ()) ; let compiler = args . compiler () . bytes (args . flag_bytes) . only_utf8 (! args . flag_bytes) . dfa (args . flag_dfa) . reverse (args . flag_dfa_reverse) ; let prog = try ! (compiler . compile (& exprs)) ; print ! ("{:?}" , prog) ; Ok (()) }
};
}
