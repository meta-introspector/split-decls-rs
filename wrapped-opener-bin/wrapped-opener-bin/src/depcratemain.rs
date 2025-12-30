// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args = Args :: from_args () ; let open_result = if args . browser { opener :: open_browser (& args . path) } else if args . reveal { opener :: reveal (& args . path) } else { opener :: open (& args . path) } ; match open_result { Ok (()) => { println ! ("Opened path successfully.") ; } Err (e) => { println ! ("Failed to open path.\n\nerror:\n\n{e:#?}") ; process :: exit (1) ; } } }
};
}
