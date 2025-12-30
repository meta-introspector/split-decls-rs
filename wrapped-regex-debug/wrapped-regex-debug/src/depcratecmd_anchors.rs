// Generated macro for cmd_anchors (function)
macro_rules! Depcratecmd_anchors {
() => {
// Module: crate
// Provides: {"cmd_anchors"}
// Dependencies: {}
fn cmd_anchors (args : & Args) -> Result < () > { let expr = try ! (args . parse_one ()) ; if expr . is_anchored_start () { println ! ("start") ; } if expr . is_anchored_end () { println ! ("end") ; } Ok (()) }
};
}
