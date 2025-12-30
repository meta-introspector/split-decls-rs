// Generated macro for color_diff (function)
macro_rules! Depcrate_log_formatcolor_diff {
() => {
// Module: crate::log::format
// Provides: {"color_diff"}
// Dependencies: {}
fn color_diff (text : String) -> Result < String , String > { let lines = text . lines () . collect :: < Vec < _ > > () ; let nlines = lines . len () ; if nlines > 2 { let left = lines [nlines - 2] ; let right = lines [nlines - 1] ; const LEFT_START : & str = " left: `" ; const RIGHT_START : & str = "right: `" ; const END : & str = "`" ; if left . starts_with (LEFT_START) && left . ends_with (END) && right . starts_with (RIGHT_START) && right . ends_with (END) { let left = & left [LEFT_START . len () .. left . len () - END . len ()] ; let right = & right [RIGHT_START . len () .. right . len () - END . len ()] ; let mut buf = lines [.. nlines - 2] . join ("\n") . bold () . to_string () ; buf . push ('\n') ; let diffs = dissimilar :: diff (left , right) ; writeln ! (buf , "{} {} / {}" , "diff" . bold () , "< left" . red () , "right >" . green ()) . ok () ; write ! (buf , "{}" , "<" . red ()) . ok () ; for diff in & diffs { match diff { Chunk :: Equal (s) => { write ! (buf , "{}" , s . red ()) . ok () ; } Chunk :: Insert (_) => continue , Chunk :: Delete (s) => { write ! (buf , "{}" , s . red () . bold ()) . ok () ; } } } buf . push ('\n') ; write ! (buf , "{}" , ">" . green ()) . ok () ; for diff in & diffs { match diff { Chunk :: Equal (s) => { write ! (buf , "{}" , s . green ()) . ok () ; } Chunk :: Delete (_) => continue , Chunk :: Insert (s) => { write ! (buf , "{}" , s . green () . bold ()) . ok () ; } } } return Ok (buf) ; } } Err (text) }
};
}
